import logging
import os

import qarax
from qarax.api import hosts_api
from qarax.model.host import Host

import terraform

log = logging.getLogger(__name__)

log.info("Applying terraform plan...")
tf = terraform.Terraform(workdir=os.path.join(
    os.path.abspath(os.path.dirname(__file__)), 'terraform'))

tf.apply()
vm_json = tf.show()

vm_ip = None
for resource in vm_json['values']['root_module']['resources']:
    if 'network_interface' in resource['values']:
        vm_ip = resource['values']['network_interface'][0]['addresses'][0]
log.info("VM IP address %s", vm_ip)

configuration = qarax.Configuration(
    host="http://localhost:3000"
)

with qarax.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = hosts_api.HostsApi(api_client)
    host = Host(
        address=vm_ip,
        host_user="root",
        name="hostato",
        password="centos",
        port=50051,
    )  # Host |

    try:
        # Create new host
        log.info("Adding host to database")
        api_response = api_instance.add_host(host)
        host_id = api_response['response']
        if host_id != 'error':
            log.info("Installing host '%s'", host_id)
            api_instance.install_host(host_id)
    except qarax.ApiException as e:
        print("Exception when calling HostsApi->add_host: %s\n" % e)
