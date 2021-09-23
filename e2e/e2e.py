import pprint
import qarax
import terraform
import os
import time

from qarax.api import hosts_api
from qarax.model.host import Host


tf = terraform.Terraform()
out, err = tf.apply(workdir=os.path.join(os.path.abspath(os.path.dirname(__file__)), 'terraform'))

vm_json = tf.show(workdir=os.path.join(os.path.abspath(os.path.dirname(__file__)), 'terraform'))
vm_ip = None
for resource in vm_json['values']['root_module']['resources']:
    if 'network_interface' in resource['values']:
        vm_ip = resource['values']['network_interface'][0]['addresses'][0]

configuration = qarax.Configuration(
    host="http://localhost:3000"
)

time.sleep(60)

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
        api_response = api_instance.add_host(host)
        host_id = api_response['response'];
        if host_id != 'error':
            pprint.pprint(host_id)
            api_instance.install_host(host_id)
    except qarax.ApiException as e:
        print("Exception when calling HostsApi->add_host: %s\n" % e)
