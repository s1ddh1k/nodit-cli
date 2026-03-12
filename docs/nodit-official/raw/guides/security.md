---
## What is Security?

Nodit provides additional security features beyond API Keys. **Security** is a feature that identifies users who can access project resources using Domain Names and Source IPs to prevent unauthorized users from using project resources and making API calls. Requests from IPs or Domains not registered in the Allowlist will receive an error response with HTTP 403 Status Code along with the following messages:

```json
{
  "code": "PERMISSION_DENIED",
  "message": "Permission denied. Check your security settings and ensure your domain is allowlisted."
}

or

{
  "code": "PERMISSION_DENIED",
  "message": "Permission denied. Check your security settings and ensure your IP is allowlisted."
}
```


## Where can I configure Security settings?

You can configure and manage Domain/IP Allowlists in the Security tab provided on the project detail screen.




### **1. Registering Domain Names**

This method blocks resource access from unauthorized Origins based on Domain Names. Here's how to add a Domain Name to the Allowlist:



* Click the toggle to the left of [Domain Allowlist] to enable security settings using Domain Names.
* You can enter a brief description of the Domain Name being registered in the [Name] box. (Optional)
* Enter the Domain Name in the [Domain Name] box.
* Click the [Add] button on the right to add the Domain Name to the Domain Name Allowlist.

You can register up to 25 Domain Names in the Allowlist. From the moment you enable the option, requests from Origins other than those registered in the allowlist will return a 403 error.

### **2. Registering Source IPs**

This method blocks resource access from unauthorized Origins based on Source IP. Here's how to add an IP to the Allowlist:



* Click the toggle to the left of [Source IP] to enable security settings using Source IP.
* You can enter a brief description of the Source IP being registered in the [Name] box. (Optional)
* Enter the IP address in the [Source IP] box.
* Click the [Add] button on the right to add the Source IP to the Source IP Allowlist.


You can register up to 25 Source IPs in the allowlist. From the moment you enable security, requests from Origins other than those registered in the allowlist will return a 403 error.

:::info Please check whether the Allowlist is enabled!
If Domain or Source IP Allowlist is enabled, calling the API from an unregistered Domain or Source IP will return a 403 error. When not using Security, you must keep the Allowlist disabled to make API calls normally.
:::