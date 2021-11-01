# Deepnet Examples

These examples provide simple coding examples of how the data in Deepnet can flow. The current capabilities are demonstrated:

- The data structure used is found in [definition.json](definition.json)
- A sample of the definition is found in [example.json](example.json)
- How to send data over MQTT to an azure iot hub is done in [publish.py](publish.py)
- How to receive data sent to the iot hub over AMQP and websockets is illustrated in [subscribe.js](subscribe.js)
- [example.bicep](example.bicep) can be run to create an iot hub and its associated resources in azure

To be able to create the resources, you must create a resource group in one of your subscriptions and then run

```powershell
az login
az account set --subscription [subscription id]
az deployment group create --name [deployment name] --resource-group [rg name] --template-file example.bicep
```

An IoT device will then have to be registered in the iot hub created by the bicep file.

To stream data to the iot hub over MQTT run

```powershell
python publish.py [iot hub name] [iot hub device name] [data.csv]
```

where data.csv contains datapoint for a simulated mission. The primary key of the device will have to be set under the environment variable `IOT_HUB_DEVICE_KEY`

To read the data sent to the iot hub, you can run

```powershell
node subscribe.js
```

You have to define the environment variable `IOT_HUB_CONNECTION_STRING` which you can find under the Built-in Endpoints under the iot hub you have created.
