---
## Nodit Status Page Guide

We provide the [Nodit Status page](https://status.nodit.io/) for Nodit customers. Any Nodit customer can use it by clicking the [Service Status] menu at the top of the developer documentation. The Nodit Status page provides the following features:

* [Real-time Node Operation Status Provision](#real-time-node-operation-status-provision)
* [Check Work Schedule](#check-work-schedule)
* [Check Recent Incident History](#check-recent-incident-history)
* [Subscribe to Event Notifications (Email, RSS, JSON Data, Webhook)](#subscribe-to-event-notifications)

<br />

### Real-time Node Operation Status Provision

In the Status menu, you can check the operation status of all Elastic Nodes and Dedicated Nodes provided by Nodit in real-time. Status pages for Dedicated Node customers are provided separately. Clicking each chain item displays the status history for that node for the past 7 days. If service degradation occurred on a specific date due to events or infrastructure work, you can check that information together.





<br />

### Check Work Schedule

You can check major maintenance work schedules that affect node services in advance by clicking the Maintenance menu, such as node client upgrades, network work for infrastructure expansion or stabilization. You can minimize service impact by referring to announced work schedules.



<br />

### Check Recent Incident History

You can check recent Incident history that occurred in the service by clicking the Previous Incident menu. You can easily check the target chain, content, occurrence date and time, and whether the issue persists at a glance.



<br />

### Subscribe to Event Notifications

You can subscribe to Nodit major work schedules or Incident status in real-time by clicking the [Get updates] button at the top right. Four subscription methods are supported: Email, RSS feed, JSON data, and Webhook.

<br />

#### Subscribe via Email

<div className="status-subscribe-section">
  <div className="status-subscribe-row">
    <div className="status-subscribe-image">
      
    
    <div className="status-subscribe-text">
      <p>Enter the email address where you want to receive Notifications in the email address input field. If you want to receive only events for specific chains, select the 'Subscribe to specific components' checkbox and then select the desired chains from the target chain list.</p>
      <p>Clicking the [Subscribe] button completes the subscription, and thereafter, when work or Incidents occur on selected chains, the content is automatically sent to the entered email address.</p>
    
  


#### RSS

<div className="status-subscribe-section">
  <div className="status-subscribe-row">
    <div className="status-subscribe-image">
      
    
    <div className="status-subscribe-text">
      <p>You can subscribe to status updates in real-time through RSS or Atom feeds. Easily integrate through RSS readers or tools that support feed subscription (Feedly, Thunderbird, Slack external apps, etc.).</p>
      <p>RSS Feed Subscription Address<br />[https://status.nodit.io/feed.rss](https://status.nodit.io/feed.rss)</p>
      <p>Atom Feed Subscription Address<br />[https://status.nodit.io/feed.atom](https://status.nodit.io/feed.atom)</p>
    
  


#### JSON

<div className="status-subscribe-section">
  <div className="status-subscribe-row">
    <div className="status-subscribe-image">
      
    
    <div className="status-subscribe-text">
      <p>If you are monitoring Nodit failure situations and linking them to services through integration with your own monitoring system, you can query the Status page in JSON format to obtain necessary data items. Status page information in JSON format can be queried at [https://status.nodit.io/index.json](https://status.nodit.io/index.json).</p>
    
  


#### Webhook

<div className="status-subscribe-section">
  <div className="status-subscribe-row">
    <div className="status-subscribe-image">
      
    
    <div className="status-subscribe-text">
      <p>This is a feature that can deliver the content to the entered Webhook URL when event Notifications occur. After checking the Webhook URL of operational tools such as operational servers or Slack channels, register the Webhook by entering it on the Nodit Status page.</p>
      <p>If there is a problem with the entered Endpoint making Webhook delivery difficult, you must also enter an email address where this can be notified. Like email subscription, you can selectively subscribe only to events related to specific chains you want to monitor.</p>