// Copyright (c) Microsoft Corporation.
// Licensed under the MIT Licence.

/*
  This sample demonstrates how to use the Microsoft Azure Event Hubs Client for JavaScript to 
  read messages sent from a device. Please see the documentation for @azure/event-hubs package
  for more details at https://www.npmjs.com/package/@azure/event-hubs

  For an example that uses checkpointing, follow up this sample with the sample in the 
  eventhubs-checkpointstore-blob package on GitHub at the following link:

  https://github.com/Azure/azure-sdk-for-js/blob/master/sdk/eventhub/eventhubs-checkpointstore-blob/samples/javascript/receiveEventsUsingCheckpointStore.js
*/

const { EventHubConsumerClient } = require("@azure/event-hubs");

// If using websockets, uncomment the following require statement
const WebSocket = require("ws");

// Validator for verifying content of received telemetry
const Validator = require("jsonschema").Validator;

// Read the Event hub compatible endpoint connection string from environment variable
const connectionString = process.env.IOT_HUB_CONNECTION_STRING;

//Define schema for message validation
const dataValidator = new Validator();
const schema = {
  "id": "/DroneData",
  "type": "object",
  "properties": {
    "MissionId": {"type": "string"},
    "Position": {
      "type": "array",
      "items": {"type": "number"}
    },
    "Orientation": {
      "type": "array",
      "items": {"type": "number"}
    }
  },
  "required": ["MissionId", "Position", "Orientation"]
};

var printError = function (err) {
  console.log(err.message);
};

// Display the message content - telemetry and properties.
// - Telemetry is sent in the message body
// - The device can add arbitrary properties to the message
// - IoT Hub adds system properties, such as Device Id, to the message.
var printMessages = function (messages) {
  for (const message of messages) {
    console.log("Telemetry received: ");
    var data = message.body;
    var validation = dataValidator.validate(data, schema);
    validation.errors.forEach(e => console.log(e.property + ": " + e.message));
    console.log(JSON.stringify(data));
    console.log("");
  }
};

async function main() {
  console.log("IoT Hub Quickstarts - Read device to cloud messages.");

  // Use Websockets
  const clientOptions = {
     webSocketOptions: {
       webSocket: WebSocket,
       webSocketConstructorOptions: {}
     }
  };
  // Create the client to connect to the default consumer group of the Event Hub
  const consumerClient = new EventHubConsumerClient("$Default", connectionString, clientOptions);

  // Subscribe to messages from all partitions as below
  // To subscribe to messages from a single partition, use the overload of the same method.
  consumerClient.subscribe({
    processEvents: printMessages,
    processError: printError,
  });
}

main().catch((error) => {
  console.error("Error running sample:", error);
});
