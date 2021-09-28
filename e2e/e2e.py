import logging
import os
import time
from functools import lru_cache

import pytest
import qarax
from qarax.api import hosts_api
from qarax.model.host import Host

import terraform

log = logging.getLogger(__name__)


@pytest.fixture
def tf():
    tf = terraform.Terraform(workdir=os.path.join(
        os.path.abspath(os.path.dirname(__file__)), 'terraform'))

    yield tf


@pytest.fixture
def vm(tf):
    _, err = tf.apply()
    if err:
        log.error(err)
        raise Exception("Failed to apply terraform plan")

    vm_json, err = tf.show()
    if err:
        log.error(err)
        raise Exception("Failed to show terraform resource")

    yield vm_json


@pytest.fixture
def vm_ip(vm):
    log.info("Getting VM IP address")
    vm_ip = None

    for resource in vm['values']['root_module']['resources']:
        if 'network_interface' in resource['values']:
            vm_ip = resource['values']['network_interface'][0]['addresses'][0]

    if vm_ip is None:
        raise Exception("VM IP address not found")

    log.info("VM IP address %s", vm_ip)
    yield vm_ip


@pytest.fixture
def qarax_configuration():
    configuration = qarax.Configuration(
        host="http://localhost:3000"
    )

    yield configuration


@pytest.fixture
def api_client(qarax_configuration):
    api_client = qarax.ApiClient(qarax_configuration)

    yield api_client


@pytest.mark.order(1)
def test_install_host(api_client, vm_ip):
    api_instance = hosts_api.HostsApi(api_client)
    host = Host(
        address=vm_ip,
        host_user="root",
        name="hostato",
        password="centos",
        port=50051,
    )

    try:
        log.info("Adding host to database")
        api_response = api_instance.add_host(host)
        host_id = api_response['response']
        if host_id != 'error':
            log.info("Installing host '%s'", host_id)
            api_instance.install_host(host_id)
            while True:
                host = api_instance.get_host(host_id)['response']

                # TODO: handle failed installation
                if host['status'] == 'up':
                    log.info("Host '%s' installed", host_id)
                    break
                time.sleep(3)
    except qarax.ApiException as e:
        log.error("Exception when calling HostsApi->add_host: %s\n" % e)

    healthcheck = api_instance.healthcheck(host_id)
    assert healthcheck['response'] == 'ok'
