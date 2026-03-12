---
<div className="dedicated-features-box">
  <div className="dedicated-features-box__image">
    
  
  <div className="dedicated-features-box__content">
    <h2>Key Features</h2>
    <div className="dedicated-features-box__text">
✅ **Real-time Blockchain Data Stream**\
You can subscribe to on-chain events needed by your application in real-time Stream, including blocks, transactions, events, and smart contract state changes. Instead of requesting data queries each time, you can continuously receive data by maintaining a connection, enabling effective monitoring.

✅ **Filter Support**\
Supports various event filters to effectively select and query only the data you need.
    
  


***

## Subscribe to Real-time Blockchain Data from Nodes with Nodit Stream

Nodit's Stream provides real-time blockchain data streaming through protocols such as WebSocket and gRPC. You can monitor project-related data or implement reactive services by defining and subscribing to the data you want through various filters. Unlike traditional HTTP API request methods, the WebSocket-based Stream feature maintains a persistent connection and receives immediate notifications whenever new data occurs, enabling faster and more efficient data processing. The following use cases are possible:

* When a transaction submitted by a user is successfully processed, you can receive the transaction Receipt and update the screen.
* When a specific user's wallet address or designated contract event (asset transfer, transaction transfer, etc.) occurs, you can send an alert to the user or perform appropriate post-processing actions.
* By receiving notifications whenever a block is created, you can monitor whether a specific network environment is operating stably.

## Nodit Stream Quickstart

Events that can be subscribed through Stream can be found on the [Stream Reference](/api/stream-event-type) page.

Follow the recipe below to subscribe to Websocket Stream and implement simple code to receive real-time data.


:::warning Stream is a feature that requires high Compute Unit (CU) usage.
When using Stream, usage (CU) is deducted in proportion to the data transfer amount (Bytes), so a large amount of usage may be deducted depending on the subscription time. Depending on the Plan you are subscribed to, sufficient Stream usage may not be possible, so please use it carefully by referring to the pricing policy below.
* 0.03 CU is deducted per 1 byte of data transmitted through Stream.
:::

:::info Ready to start for free right now?
Visit the <a href="https://nodit.lambda256.io/?utm_source=portal&utm_medium=docs&utm_campaign=stream">Nodit Console</a> to create an account and try out the features in a free project.
:::