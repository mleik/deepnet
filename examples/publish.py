import sys
import os
import json
import csv
import ssl
from base64 import b64decode, b64encode
from hashlib import sha256
import time
from urllib import parse
from hmac import HMAC

from paho.mqtt import client as mqtt

def generate_device_sas_token(uri, device, key, policy_name, expiry=3600):
    ttl = time.time() + expiry
    sign_key = '%s\n%d' % ((parse.quote_plus(uri+'%2Fdevices%2F'+device)), int(ttl))
    signature = b64encode(HMAC(b64decode(key), sign_key.encode('utf-8'), sha256).digest())

    raw_token = {
        'sr': uri + '%2Fdevices%2F' + device,
        'sig': signature,
        'se': str(int(ttl))
    }
    if policy_name is not None:
        raw_token['skn'] = policy_name

    return 'SharedAccessSignature ' + parse.urlencode(raw_token)

def on_connect(client, userdata, flags, rc):
    print("Device connected with result code: " + str(rc))


def on_disconnect(client, userdata, rc):
    print("Device disconnected with result code: " + str(rc))


def on_publish(client, userdata, mid):
    print("Device sent message")


key = os.environ.get("IOT_HUB_DEVICE_KEY")
iot_hub_name = sys.argv[1]
device_id = sys.argv[2]
sas_token = generate_device_sas_token(iot_hub_name + '.azure-devices.net',device_id, key, None)

file_name = sys.argv[3]

client = mqtt.Client(client_id=device_id, protocol=mqtt.MQTTv311, clean_session=0)

client.on_connect = on_connect
client.on_disconnect = on_disconnect
client.on_publish = on_publish

client.username_pw_set(
    username=iot_hub_name
    + ".azure-devices.net/"
    + device_id
    + "/?api-version=2018-06-30",
    password=sas_token,
)

client.tls_set(
    certfile=None,
    keyfile=None,
    cert_reqs=ssl.CERT_REQUIRED,
    tls_version=ssl.PROTOCOL_TLSv1_2,
    ciphers=None,
)
client.tls_insecure_set(False)
client.reconnect_delay_set(5)
client.connect(iot_hub_name + ".azure-devices.net", port=8883)

with open(file_name, newline='') as mission_data:
    reader = csv.DictReader(mission_data)
    i = 60
    j=0
    client.loop_start()
    for row in reader:
        j+=1
        if i==60:
            data = {"MissionId": "TestMission", "Position": [float(row['Vehicle Y']), float(row['Vehicle X']), float(row['Depth'])], "Orientation": [float(row['Heading']), 0.0, 0.0]}
            response = client.publish("devices/" + device_id + "/messages/events/", bytes(json.dumps(data), 'utf-8'), qos=1)
            response.wait_for_publish()
            print(json.dumps(data))
            print(j)
            i=0
        else:
            i+=1
        time.sleep(0.1)
client.disconnect()
