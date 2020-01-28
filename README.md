# cloudflare-rs
> Rust library for the Cloudflare v4 API

[![cloudflare's crates.io badge](https://img.shields.io/crates/v/cloudflare.svg)](https://crates.io/crates/cloudflare)
[![cloudflare's docs.rs badge](https://docs.rs/cloudflare/badge.svg)](https://docs.rs/cloudflare)

⚠️ This library is a Work in Progess! ⚠️

This library provides convenience functions that wrap the Cloudflare API.

## Implemented Endpoints
[User](https://api.cloudflare.com/#user-properties) - The currently logged in/authenticated User
- [X] [User Details](https://api.cloudflare.com/#user-user-details)
- [ ] [Edit Details](https://api.cloudflare.com/#user-edit-user) - Edit part of your user details

[User's Account Memberships](https://api.cloudflare.com/#user-s-account-memberships-properties) - A list of memberships of accounts this user can access
- [ ] [List Memberships](https://api.cloudflare.com/#user-s-account-memberships-list-memberships) - List memberships of accounts the user can access
- [ ] [Membership Details](https://api.cloudflare.com/#user-s-account-memberships-membership-details) - Get a specific membership
- [ ] [Update Membership](https://api.cloudflare.com/#user-s-account-memberships-update-membership) - Accept or reject this account invitation
- [ ] [Delete Membership](https://api.cloudflare.com/#user-s-account-memberships-delete-membership) - Remove the associated member from an account

[Accounts](https://api.cloudflare.com/#accounts-properties) - An Account is the root object which owns other resources such as zones, load balancers and billing details
- [X] [List Accounts](https://api.cloudflare.com/#accounts-list-accounts) - List all accounts you have ownership or verified access to
- [ ] [Account Details](https://api.cloudflare.com/#accounts-account-details) - Get information about a specific account that you are a member of
- [ ] [Update Account](https://api.cloudflare.com/#accounts-update-account) - Update an existing Account

[Account Members](https://api.cloudflare.com/#account-members-properties) - An association between a Cloudflare user and an account
- [ ] [List Members](https://api.cloudflare.com/#account-members-list-members) - List all members of an account
- [ ] [Add Member](https://api.cloudflare.com/#account-members-add-member) - Add a user to the list of members for this account
- [ ] [Member Details](https://api.cloudflare.com/#account-members-member-details) - Get information about a specific member of an account
- [ ] [Update Member](https://api.cloudflare.com/#account-members-update-member) - Modify an account member
- [ ] [Remove Member](https://api.cloudflare.com/#account-members-remove-member) - Remove a member from an account

[Account Roles](https://api.cloudflare.com/#account-roles-properties) - A Role defines what permissions a Member of an Account has
- [ ] [List Roles](https://api.cloudflare.com/#account-roles-list-roles) - Get all available roles for an account
- [ ] [Role Details](https://api.cloudflare.com/#account-roles-role-details) - Get information about a specific role for an account

[Account Subscriptions](https://api.cloudflare.com/#account-subscriptions-properties) - Listing of an account's subscriptions
- [ ] [List Subscriptions](https://api.cloudflare.com/#account-subscriptions-list-subscriptions) - Lists all an account's subscriptions
- [ ] [Create Subscription](https://api.cloudflare.com/#account-subscriptions-create-subscription) - Create an account subscription
- [ ] [Update Subscription](https://api.cloudflare.com/#account-subscriptions-update-subscription) - Update an account subscriptions
- [ ] [Delete Subscription](https://api.cloudflare.com/#account-subscriptions-delete-subscription) - Deletes an account's subscription

[Organizations](https://api.cloudflare.com/#organizations-properties) **(Deprecated from 4-FEB-2020)** - An Organization is an entity which holds a set of zones for multiple users to interact with
- [ ] [Organization Details](https://api.cloudflare.com/#organizations-organization-details) - Get information about a specific organization that you are a member of
- [ ] [Edit Organization](https://api.cloudflare.com/#organizations-edit-organization) - Update an existing Organization

[Organization Invites](https://api.cloudflare.com/#organization-invites-properties) **(Deprecated from 4-FEB-2020)** - Invitations to potential members that this organization has created
- [ ] [List Invitations](https://api.cloudflare.com/#organization-invites-list-invitations) - List all invitations associated with an organization
- [ ] [Create Invitation](https://api.cloudflare.com/#organization-invites-create-invitation) - Invite a User to become a Member of an Organization
- [ ] [Invitation Details](https://api.cloudflare.com/#organization-invites-invitation-details) - Get the details of an invitation
- [ ] [Edit Invitation Roles](https://api.cloudflare.com/#organization-invites-edit-invitation-roles) - Change the Roles of a Pending Invite
- [ ] [Cancel Invitation](https://api.cloudflare.com/#organization-invites-cancel-invitation) - Cancel an existing invitation

[Organization Members](https://api.cloudflare.com/#organization-members-properties) **(Deprecated from 4-FEB-2020)** - A member is the association of a Cloudflare user with an Organization
- [ ] [List Members](https://api.cloudflare.com/#organization-members-list-members) - List all members of a organization
- [ ] [Member Details](https://api.cloudflare.com/#organization-members-member-details) - Get information about a specific member of an organization
- [ ] [Edit Member Roles](https://api.cloudflare.com/#organization-members-edit-member-roles) - Change the Roles of an Organization's Member
- [ ] [Remove Member](https://api.cloudflare.com/#organization-members-remove-member) - Remove a member from an organization

[Organization Roles](https://api.cloudflare.com/#organization-roles-properties) **(Deprecated from 4-FEB-2020)** - A role defines what permissions a Member of an Organization has
- [ ] [List Roles](https://api.cloudflare.com/#organization-roles-list-roles) - Get all available roles for an organization
- [ ] [Role Details](https://api.cloudflare.com/#organization-roles-role-details) - Get information about a specific role for an organization

[User's Invites](https://api.cloudflare.com/#user-s-invites-properties) - Your pending invitations to organizations
- [ ] [List Invitations](https://api.cloudflare.com/#user-s-invites-list-invitations) - List all invitations associated with my user
- [ ] [Invitation Details](https://api.cloudflare.com/#user-s-invites-invitation-details) - Get the details of an invitation
- [ ] [Respond to Invitation](https://api.cloudflare.com/#user-s-invites-respond-to-invitation) - Respond to an invitation

[User's Organizations](https://api.cloudflare.com/#user-s-organizations-properties) - A list of organizations this user is a member of
- [ ] [List Organizations](https://api.cloudflare.com/#user-s-organizations-list-organizations) - List organizations the user is associated with
- [ ] [Organization Details](https://api.cloudflare.com/#user-s-organizations-organization-details) - Get a specific organization the user is associated with
- [ ] [Leave Organization](https://api.cloudflare.com/#user-s-organizations-leave-organization) - Remove association to an organization

[User Billing Profile](https://api.cloudflare.com/#user-billing-profile-properties) - A user billing profile
- [ ] [Billing Profile Details](https://api.cloudflare.com/#user-billing-profile-billing-profile-details) - Access your billing profile object

[User Billing History](https://api.cloudflare.com/#user-billing-history-properties) - A user billing history
- [ ] [Billing History Details](https://api.cloudflare.com/#user-billing-history-billing-history-details) - Access your billing history object

[User Subscription](https://api.cloudflare.com/#user-subscription-properties) - Listing of a user's subscriptions
- [ ] [Get User Subscriptions](https://api.cloudflare.com/#user-subscription-get-user-subscriptions) - Lists all a user's subscriptions
- [ ] [Update User Subscription](https://api.cloudflare.com/#user-subscription-update-user-subscription) - Update a user subscriptions
- [ ] [Delete User Subscription](https://api.cloudflare.com/#user-subscription-delete-user-subscription) - Deletes a user's subscription

[Account Billing Profile](https://api.cloudflare.com/#account-billing-profile-properties) - Account's billing profile
- [ ] [Billing Profile Details](https://api.cloudflare.com/#account-billing-profile-billing-profile-details) - Get the current billing profile for the account

[Zone Rate Plan](https://api.cloudflare.com/#zone-rate-plan-properties) - A zone rate plan from the billing service
- [ ] [List Available Rate Plans](https://api.cloudflare.com/#zone-rate-plan-list-available-rate-plans) - List all rate plans the zone can subscribe to
- [ ] [List Available Plans](https://api.cloudflare.com/#zone-rate-plan-list-available-plans) - List available plans the zone can subscribe to
- [ ] [Available Plan Details](https://api.cloudflare.com/#zone-rate-plan-available-plan-details) - Details of an available plan that the zone can subscribe to

[Zone Subscription](https://api.cloudflare.com/#zone-subscription-properties) - A subscription associated with a zone containing plan and add-ons
- [ ] [Zone Subscription Details](https://api.cloudflare.com/#zone-subscription-zone-subscription-details) - Zone subscription details
- [ ] [Update Zone Subscription](https://api.cloudflare.com/#zone-subscription-update-zone-subscription) - Update Zone subscriptions. Either plan and add-ons
- [ ] [Create Zone Subscription](https://api.cloudflare.com/#zone-subscription-create-zone-subscription) - Create a Zone subscriptions. Either plan or add-ons

[Audit Logs](https://api.cloudflare.com/#audit-logs-properties) - A log of changes made to your Cloudflare account
- [ ] [List User Audit Logs](https://api.cloudflare.com/#audit-logs-list-user-audit-logs) - Get audit logs for a user account, filter by who made the change, which zone was the change was made on, and the timeframe of the change
- [ ] [List Organization Audit Logs](https://api.cloudflare.com/#audit-logs-list-organization-audit-logs) - Get audit logs for an organization, filter by who made the change, which zone was the change was made on, and the timeframe of the change

[Argo Smart Routing](https://api.cloudflare.com/#argo-smart-routing-properties) - Query, enable and disable Argo smart routing for a zone
- [ ] [Get Argo Smart Routing setting](https://api.cloudflare.com/#argo-smart-routing-get-argo-smart-routing-setting)
- [ ] [Patch Argo Smart Routing setting](https://api.cloudflare.com/#argo-smart-routing-patch-argo-smart-routing-setting)

[Argo Analytics for Zone](https://api.cloudflare.com/#argo-analytics-for-zone-properties) - Argo Smart Routing Analytics for a zone
- [ ] [Argo Analytics for a zone](https://api.cloudflare.com/#argo-analytics-for-zone-argo-analytics-for-a-zone)

[Argo Analytics for Geolocation](https://api.cloudflare.com/#argo-analytics-for-geolocation-properties) - Argo Smart Routing Analytics for a zone at different PoPs
- [ ] [Argo Analytics for a zone at different PoPs](https://api.cloudflare.com/#argo-analytics-for-geolocation-argo-analytics-for-a-zone-at-differnt-pops)

[Zone](https://api.cloudflare.com/#zone-properties) - A Zone is a domain name along with its subdomains and other identities
- [X] [List Zones](https://api.cloudflare.com/#zone-list-zones) - List, search, sort, and filter your zones
- [ ] [Create Zone](https://api.cloudflare.com/#zone-create-zone)
- [X] [Zone Details](https://api.cloudflare.com/#zone-zone-details)
- [ ] [Edit Zone](https://api.cloudflare.com/#zone-edit-zone) - Only one zone property can be changed at a time
- [ ] [Delete Zone](https://api.cloudflare.com/#zone-delete-zone) - Delete an existing zone
- [ ] [Zone Activation Check](https://api.cloudflare.com/#zone-zone-activation-check) - Initiate another zone activation check
- [ ] [Purge All Files](https://api.cloudflare.com/#zone-purge-all-files) - Remove ALL files from Cloudflare's cache
- [ ] [Purge Files by URL](https://api.cloudflare.com/#zone-purge-files-by-url) - Granularly remove one or more files from Cloudflare's cache either by specifying URLs. All tiers can purge by URL
- [ ] [Purge Files by Cache-Tags or Host](https://api.cloudflare.com/#zone-purge-files-by-cache-tags-or-host) - Granularly remove one or more files from Cloudflare's cache either by specifying the host or the associated Cache-Tag

[Zone Settings](https://api.cloudflare.com/#zone-settings-properties) - A Zone setting changes how the Zone works in relation to caching, security, or other features of Cloudflare
- [ ] [Get all Zone settings](https://api.cloudflare.com/#zone-settings-get-all-zone-settings) - Available settings for your user in relation to a zone
- [ ] [Get Advanced DDOS setting](https://api.cloudflare.com/#zone-settings-get-advanced-ddos-setting) - Advanced protection from Distributed Denial of Service (DDoS) attacks on your website. This is an uneditable value that is 'on' in the case of Business and Enterprise zones
- [ ] [Get Always Online setting](https://api.cloudflare.com/#zone-settings-get-always-online-setting) - When enabled, Always Online will serve pages from our cache if your server is offline
- [ ] [Get Always Use HTTPS setting](https://api.cloudflare.com/#zone-settings-get-always-use-https-setting) - Reply to all requests for URLs that use "http" with a 301 redirect to the equivalent "https" URL. If you only want to redirect for a subset of requests, consider creating an "Always use HTTPS" page rule
- [ ] [Get Opportunistic Onion setting](https://api.cloudflare.com/#zone-settings-get-opportunistic-onion-setting) - Add an Alt-Svc header to all legitimate requests from Tor, allowing the connection to use our onion services instead of exit nodes
- [ ] [Get Automatic HTTPS Rewrites setting](https://api.cloudflare.com/#zone-settings-get-automatic-https-rewrites-setting) - Enable the Automatic HTTPS Rewrites feature for this zone
- [ ] [Get Browser Cache TTL setting](https://api.cloudflare.com/#zone-settings-get-browser-cache-ttl-setting) - Browser Cache TTL (in seconds) specifies how long Cloudflare-cached resources will remain on your visitors' computers. Cloudflare will honor any larger times specified by your server
- [ ] [Get Browser Check setting](https://api.cloudflare.com/#zone-settings-get-browser-check-setting) - Browser Integrity Check is similar to Bad Behavior and looks for common HTTP headers abused most commonly by spammers and denies access to your page. It will also challenge visitors that do not have a user agent or a non standard user agent (also commonly used by abuse bots, crawlers or visitors).
- [ ] [Get Cache Level setting](https://api.cloudflare.com/#zone-settings-get-cache-level-setting) - Cache Level functions based off the setting level. The basic setting will cache most static resources (i.e., css, images, and JavaScript). The simplified setting will ignore the query string when delivering a cached resource. The aggressive setting will cache all static resources, including ones with a query string.
- [ ] [Get Challenge TTL setting](https://api.cloudflare.com/#zone-settings-get-challenge-ttl-setting) - Specify how long a visitor is allowed access to your site after successfully completing a challenge (such as a CAPTCHA). After the TTL has expired the visitor will have to complete a new challenge. We recommend a 15 - 45 minute setting and will attempt to honor any setting above 45 minutes.
- [ ] [Get Development Mode setting](https://api.cloudflare.com/#zone-settings-get-development-mode-setting) - Development Mode temporarily allows you to enter development mode for your websites if you need to make changes to your site. This will bypass Cloudflare's accelerated cache and slow down your site, but is useful if you are making changes to cacheable content (like images, css, or JavaScript) and would like to see those changes right away. Once entered, development mode will last for 3 hours and then automatically toggle off.
- [ ] [Get Email Obfuscation setting](https://api.cloudflare.com/#zone-settings-get-email-obfuscation-setting) - Encrypt email adresses on your web page from bots, while keeping them visible to humans.
- [ ] [Get Hotlink Protection setting](https://api.cloudflare.com/#zone-settings-get-hotlink-protection-setting) - When enabled, the Hotlink Protection option ensures that other sites cannot suck up your bandwidth by building pages that use images hosted on your site. Anytime a request for an image on your site hits Cloudflare, we check to ensure that it's not another site requesting them. People will still be able to download and view images from your page, but other sites won't be able to steal them for use on their own pages.
- [ ] [Get IP Geolocation setting](https://api.cloudflare.com/#zone-settings-get-ip-geolocation-setting) - Enable IP Geolocation to have Cloudflare geolocate visitors to your website and pass the country code to you.
- [ ] [Get IPv6 setting](https://api.cloudflare.com/#zone-settings-get-ipv6-setting) - Enable IPv6 on all subdomains that are Cloudflare enabled.
- [ ] [Get Minify setting](https://api.cloudflare.com/#zone-settings-get-minify-setting) - Automatically minify certain assets for your website
- [ ] [Get Mobile Redirect setting](https://api.cloudflare.com/#zone-settings-get-mobile-redirect-setting) - Automatically redirect visitors on mobile devices to a mobile-optimized subdomain
- [ ] [Get Mirage setting](https://api.cloudflare.com/#zone-settings-get-mirage-setting) - Automatically optimize image loading for website visitors on mobile devices
- [ ] [Get Enable Error Pages On setting](https://api.cloudflare.com/#zone-settings-get-enable-error-pages-on-setting) - Cloudflare will proxy customer error pages on any 502,504 errors on origin server instead of showing a default Cloudflare error page. This does not apply to 522 errors and is limited to Enterprise Zones.
- [ ] [Get Opportunistic Encryption settin](https://api.cloudflare.com/#zone-settings-get-opportunistic-encryption-setting) - Enable the Opportunistic Encryption feature for this zone.
- [ ] [Get Polish setting](https://api.cloudflare.com/#zone-settings-get-polish-setting) - Strips metadata and compresses your images for faster page load times. Basic (Lossless): Reduce the size of PNG, JPEG, and GIF files - no impact on visual quality. Basic + JPEG (Lossy): Further reduce the size of JPEG files for faster image loading. Larger JPEGs are converted to progressive images, loading a lower-resolution image first and ending in a higher-resolution version. Not recommended for hi-res photography sites.
- [ ] [Get WebP setting](https://api.cloudflare.com/#zone-settings-get-webp-setting) - When the client requesting the image supports the WebP image codec, Cloudflare will serve a WebP version of the image when WebP offers a performance advantage over the original image format.
- [ ] [Get Brotli setting](https://api.cloudflare.com/#zone-settings-get-brotli-setting) - When the client requesting an asset supports the brotli compression algorithm, Cloudflare will serve a brotli compressed version of the asset.
- [ ] [Get Prefetch Preload setting](https://api.cloudflare.com/#zone-settings-get-prefetch-preload-setting) - Cloudflare will prefetch any URLs that are included in the response headers. This is limited to Enterprise Zones.
- [ ] [Get Privacy Pass setting](https://api.cloudflare.com/#zone-settings-get-privacy-pass-setting) - Privacy Pass is a browser extension developed by the Privacy Pass Team to improve the browsing experience for your visitors. Enabling Privacy Pass will reduce the number of CAPTCHAs shown to your visitors.
- [ ] [Get Response Buffering setting](https://api.cloudflare.com/#zone-settings-get-response-buffering-setting) - Enables or disables buffering of responses from the proxied server. Cloudflare may buffer the whole payload to deliver it at once to the client versus allowing it to be delivered in chunks. By default, the proxied server streams directly and is not buffered by Cloudflare. This is limited to Enterprise Zones.
- [ ] [Get Rocket Loader setting](https://api.cloudflare.com/#zone-settings-get-rocket-loader-setting) - Rocket Loader is a general-purpose asynchronous JavaScript optimisation which prioritises the rendering of your content while loading your site's Javascript asynchronously. Turning on Rocket Loader will immediately improve a web page's rendering time sometimes measured as Time to First Paint (TTFP) and also the window.onload time (assuming there is JavaScript on the page), which can have a positive impact on your Google search ranking. When turned on, Rocket Loader will automatically defer the loading of all Javascript referenced in your HTML, with no configuration required.
- [ ] [Get Security Header (HSTS) setting](https://api.cloudflare.com/#zone-settings-get-security-header-hsts-setting) - Cloudflare security header for a zone.
- [ ] [Get Security Level setting](https://api.cloudflare.com/#zone-settings-get-security-level-setting) - Choose the appropriate security profile for your website, which will automatically adjust each of the security settings. If you choose to customize an individual security setting, the profile will become Custom.
- [ ] [Get Server Side Exclude setting](https://api.cloudflare.com/#zone-settings-get-server-side-exclude-setting) - If there is sensitive content on your website that you want visible to real visitors, but that you want to hide from suspicious visitors, all you have to do is wrap the content with Cloudflare SSE tags. Wrap any content that you want to be excluded from suspicious visitors in the following SSE tags: <!--sse--><!--/sse-->. For example: <!--sse--> Bad visitors won't see my phone number, 555-555-5555 <!--/sse-->. Note: SSE only will work with HTML. If you have HTML minification enabled, you won't see the SSE tags in your HTML source when it's served through Cloudflare. SSE will still function in this case, as Cloudflare's HTML minification and SSE functionality occur on-the-fly as the resource moves through our network to the visitor's computer.
- [ ] [Get Enable Query String Sort setting](https://api.cloudflare.com/#zone-settings-get-enable-query-string-sort-setting) - Cloudflare will treat files with the same query strings as the same file in cache, regardless of the order of the query strings. This is limited to Enterprise Zones.
- [ ] [Get SSL setting](https://api.cloudflare.com/#zone-settings-get-ssl-setting) - SSL encrypts your visitor's connection and safeguards credit card numbers and other personal data to and from your website. SSL can take up to 5 minutes to fully activate. Requires Cloudflare active on your root domain or www domain. Off: no SSL between the visitor and Cloudflare, and no SSL between Cloudflare and your web server (all HTTP traffic). Flexible: SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, but no SSL between Cloudflare and your web server. You don't need to have an SSL cert on your web server, but your vistors will still see the site as being HTTPS enabled. Full: SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, and SSL between Cloudflare and your web server. You'll need to have your own SSL cert or self-signed cert at the very least. Full (Strict): SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, and SSL between Cloudflare and your web server. You'll need to have a valid SSL certificate installed on your web server. This certificate must be signed by a certificate authority, have an expiration date in the future, and respond for the request domain name (hostname).
- [ ] [Get Minimum TLS Version setting](https://api.cloudflare.com/#zone-settings-get-minimum-tls-version-setting) - Only accept HTTPS requests that use at least the TLS protocol version specified. For example, if TLS 1.1 is selected, TLS 1.0 connections will be rejected, while 1.1, 1.2, and 1.3 (if enabled) will be permitted.
- [ ] [Get Ciphers setting](https://api.cloudflare.com/#zone-settings-get-ciphers-setting) - A whitelist of ciphers for TLS termination. These ciphers must be in the BoringSSL format.
- [ ] [Get Zone Enable TLS 1.3 setting](https://api.cloudflare.com/#zone-settings-get-zone-enable-tls-1.3-setting) - Enable Crypto TLS 1.3 feature for this zone.
- [ ] [Get TLS Client Auth setting](https://api.cloudflare.com/#zone-settings-get-tls-client-auth-setting) - TLS Client Auth requires Cloudflare to connect to your origin server using a client certificate (Enterprise Only)
- [ ] [Get True Client IP setting](https://api.cloudflare.com/#zone-settings-get-true-client-ip-setting) - Allows customer to continue to use True Client IP (Akamai feature) in the headers we send to the origin. This is limited to Enterprise Zones.
- [ ] [Get Web Application Firewall (WAF) setting](https://api.cloudflare.com/#zone-settings-get-web-application-firewall-waf-setting) - The WAF examines HTTP requests to your website. It inspects both GET and POST requests and applies rules to help filter out illegitimate traffic from legitimate website visitors. The Cloudflare WAF inspects website addresses or URLs to detect anything out of the ordinary. If the Cloudflare WAF determines suspicious user behavior, then the WAF will 'challenge' the web visitor with a page that asks them to submit a CAPTCHA successfully to continue their action. If the challenge is failed, the action will be stopped. What this means is that Cloudflare's WAF will block any traffic identified as illegitimate before it reaches your origin web server.
- [ ] [Get HTTP2 setting](https://api.cloudflare.com/#zone-settings-get-http2-setting) - Value of the HTTP2 setting
- [ ] [Get HTTP3 setting](https://api.cloudflare.com/#zone-settings-get-http3-setting) - Value of the HTTP3 setting
- [ ] [Get 0-RTT session resumption setting](https://api.cloudflare.com/#zone-settings-get-0-rtt-session-resumption-setting) - Value of the 0-RTT setting
- [ ] [Get Pseudo IPv4 setting](https://api.cloudflare.com/#zone-settings-get-pseudo-ipv4-setting) - Value of the Pseudo IPv4 setting
- [ ] [Get WebSockets setting](https://api.cloudflare.com/#zone-settings-get-websockets-setting) - WebSockets are open connections sustained between the client and the origin server. Inside a WebSockets connection, the client and the origin can pass data back and forth without having to reestablish sessions. This makes exchanging data within a WebSockets connection fast. WebSockets are often used for real-time applications such as live chat and gaming.
- [ ] [Get Image Resizing setting](https://api.cloudflare.com/#zone-settings-get-image-resizing-setting) - Image Resizing provides on-demand resizing, conversion and optimisation for images served through Cloudflare's network.
- [ ] [Get HTTP/2 Edge Prioritization setting](https://api.cloudflare.com/#zone-settings-get-http/2-edge-prioritization-setting) - HTTP/2 Edge Prioritization optimises the delivery of resources served through HTTP/2 to improve page load performance. It also supports fine control of content delivery when used in conjunction with Workers.
- [ ] [Edit zone settings info](https://api.cloudflare.com/#zone-settings-edit-zone-settings-info) - Edit settings for a zone
- [ ] [Change Always Online setting](https://api.cloudflare.com/#zone-settings-change-always-online-setting) - When enabled, Always Online will serve pages from our cache if your server is offline
- [ ] [Change Always Use HTTPS setting](https://api.cloudflare.com/#zone-settings-change-always-use-https-setting) - Reply to all requests for URLs that use "http" with a 301 redirect to the equivalent "https" URL. If you only want to redirect for a subset of requests, consider creating an "Always use HTTPS" page rule.
- [ ] [Change Opportunistic Onion setting](https://api.cloudflare.com/#zone-settings-change-opportunistic-onion-setting) - Add an Alt-Svc header to all legitimate requests from Tor, allowing the connection to use our onion services instead of exit nodes.
- [ ] [Change Automatic HTTPS Rewrites setting](https://api.cloudflare.com/#zone-settings-change-automatic-https-rewrites-setting) - Enable the Automatic HTTPS Rewrites feature for this zone.
- [ ] [Change Browser Cache TTL setting](https://api.cloudflare.com/#zone-settings-change-browser-cache-ttl-setting) - Browser Cache TTL (in seconds) specifies how long Cloudflare-cached resources will remain on your visitors' computers. Cloudflare will honor any larger times specified by your server.
- [ ] [Change Browser Check setting](https://api.cloudflare.com/#zone-settings-change-browser-check-setting) - Browser Integrity Check is similar to Bad Behavior and looks for common HTTP headers abused most commonly by spammers and denies access to your page. It will also challenge visitors that do not have a user agent or a non standard user agent (also commonly used by abuse bots, crawlers or visitors).
- [ ] [Change Cache Level setting](https://api.cloudflare.com/#zone-settings-change-cache-level-setting) - Cache Level functions based off the setting level. The basic setting will cache most static resources (i.e., css, images, and JavaScript). The simplified setting will ignore the query string when delivering a cached resource. The aggressive setting will cache all static resources, including ones with a query string.
- [ ] [Change Challenge TTL setting](https://api.cloudflare.com/#zone-settings-change-challenge-ttl-setting) - Specify how long a visitor is allowed access to your site after successfully completing a challenge (such as a CAPTCHA). After the TTL has expired the visitor will have to complete a new challenge. We recommend a 15 - 45 minute setting and will attempt to honor any setting above 45 minutes.
- [ ] [Change Development Mode setting](https://api.cloudflare.com/#zone-settings-change-development-mode-setting) - Development Mode temporarily allows you to enter development mode for your websites if you need to make changes to your site. This will bypass Cloudflare's accelerated cache and slow down your site, but is useful if you are making changes to cacheable content (like images, css, or JavaScript) and would like to see those changes right away. Once entered, development mode will last for 3 hours and then automatically toggle off.
- [ ] [Change Email Obfuscation setting](https://api.cloudflare.com/#zone-settings-change-email-obfuscation-setting) - Encrypt email adresses on your web page from bots, while keeping them visible to humans.
- [ ] [Change Enable Error Pages On setting](https://api.cloudflare.com/#zone-settings-change-enable-error-pages-on-setting) - Cloudflare will proxy customer error pages on any 502,504 errors on origin server instead of showing a default Cloudflare error page. This does not apply to 522 errors and is limited to Enterprise Zones.
- [ ] [Change Enable Query String Sort setting](https://api.cloudflare.com/#zone-settings-change-enable-query-string-sort-setting) - Cloudflare will treat files with the same query strings as the same file in cache, regardless of the order of the query strings. This is limited to Enterprise Zones.
- [ ] [Change Hotlink Protection setting](https://api.cloudflare.com/#zone-settings-change-hotlink-protection-setting) - When enabled, the Hotlink Protection option ensures that other sites cannot suck up your bandwidth by building pages that use images hosted on your site. Anytime a request for an image on your site hits Cloudflare, we check to ensure that it's not another site requesting them. People will still be able to download and view images from your page, but other sites won't be able to steal them for use on their own pages.
- [ ] [Change IP Geolocation setting](https://api.cloudflare.com/#zone-settings-change-ip-geolocation-setting) - Enable IP Geolocation to have Cloudflare geolocate visitors to your website and pass the country code to you.
- [ ] [Change IPv6 setting](https://api.cloudflare.com/#zone-settings-change-ipv6-setting) - Enable IPv6 on all subdomains that are Cloudflare enabled.
- [ ] [Change Minify setting](https://api.cloudflare.com/#zone-settings-change-minify-setting) - Automatically minify certain assets for your website
- [ ] [Change Mobile Redirect setting](https://api.cloudflare.com/#zone-settings-change-mobile-redirect-setting) - Automatically redirect visitors on mobile devices to a mobile-optimized subdomain
- [ ] [Change Mirage setting](https://api.cloudflare.com/#zone-settings-change-mirage-setting) - Automatically optimize image loading for website visitors on mobile devices
- [ ] [Change Opportunistic Encryption setting](https://api.cloudflare.com/#zone-settings-change-opportunistic-encryption-setting) - Enable the Opportunistic Encryption feature for this zone.
- [ ] [Change Polish setting](https://api.cloudflare.com/#zone-settings-change-polish-setting) - Strips metadata and compresses your images for faster page load times. Basic (Lossless): Reduce the size of PNG, JPEG, and GIF files - no impact on visual quality. Basic + JPEG (Lossy): Further reduce the size of JPEG files for faster image loading. Larger JPEGs are converted to progressive images, loading a lower-resolution image first and ending in a higher-resolution version. Not recommended for hi-res photography sites.
- [ ] [Change WebP setting](https://api.cloudflare.com/#zone-settings-change-webp-setting) - When the client requesting the image supports the WebP image codec, Cloudflare will serve a WebP version of the image when WebP offers a performance advantage over the original image format.
- [ ] [Change Brotli setting](https://api.cloudflare.com/#zone-settings-change-brotli-setting) - When the client requesting an asset supports the brotli compression algorithm, Cloudflare will serve a brotli compressed version of the asset.
- [ ] [Change Prefetch Preload setting](https://api.cloudflare.com/#zone-settings-change-prefetch-preload-setting) - Cloudflare will prefetch any URLs that are included in the response headers. This is limited to Enterprise Zones.
- [ ] [Change Privacy Pass setting](https://api.cloudflare.com/#zone-settings-change-privacy-pass-setting) - Privacy Pass is a browser extension developed by the Privacy Pass Team to improve the browsing experience for your visitors. Enabling Privacy Pass will reduce the number of CAPTCHAs shown to your visitors.
- [ ] [Change Response Buffering setting](https://api.cloudflare.com/#zone-settings-change-response-buffering-setting) - Enables or disables buffering of responses from the proxied server. Cloudflare may buffer the whole payload to deliver it at once to the client versus allowing it to be delivered in chunks. By default, the proxied server streams directly and is not buffered by Cloudflare. This is limited to Enterprise Zones.
- [ ] [Change Rocket Loader setting](https://api.cloudflare.com/#zone-settings-change-rocket-loader-setting) - Rocket Loader is a general-purpose asynchronous JavaScript optimisation which prioritises the rendering of your content while loading your site's Javascript asynchronously. Turning on Rocket Loader will immediately improve a web page's rendering time sometimes measured as Time to First Paint (TTFP) and also the window.onload time (assuming there is JavaScript on the page), which can have a positive impact on your Google search ranking. When turned on, Rocket Loader will automatically defer the loading of all Javascript referenced in your HTML, with no configuration required.
- [ ] [Change Security Header (HSTS) setting](https://api.cloudflare.com/#zone-settings-change-security-header-hsts-setting) - Cloudflare security header for a zone.
- [ ] [Change Security Level setting](https://api.cloudflare.com/#zone-settings-change-security-level-setting) - Choose the appropriate security profile for your website, which will automatically adjust each of the security settings. If you choose to customize an individual security setting, the profile will become Custom.
- [ ] [Change Server Side Exclude setting](https://api.cloudflare.com/#zone-settings-change-server-side-exclude-setting) - If there is sensitive content on your website that you want visible to real visitors, but that you want to hide from suspicious visitors, all you have to do is wrap the content with Cloudflare SSE tags. Wrap any content that you want to be excluded from suspicious visitors in the following SSE tags: <!--sse--><!--/sse-->. For example: <!--sse--> Bad visitors won't see my phone number, 555-555-5555 <!--/sse-->. Note: SSE only will work with HTML. If you have HTML minification enabled, you won't see the SSE tags in your HTML source when it's served through Cloudflare. SSE will still function in this case, as Cloudflare's HTML minification and SSE functionality occur on-the-fly as the resource moves through our network to the visitor's computer.
- [ ] [Change SSL setting](https://api.cloudflare.com/#zone-settings-change-ssl-setting) - SSL encrypts your visitor's connection and safeguards credit card numbers and other personal data to and from your website. SSL can take up to 5 minutes to fully activate. Requires Cloudflare active on your root domain or www domain. Off: no SSL between the visitor and Cloudflare, and no SSL between Cloudflare and your web server (all HTTP traffic). Flexible: SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, but no SSL between Cloudflare and your web server. You don't need to have an SSL cert on your web server, but your vistors will still see the site as being HTTPS enabled. Full: SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, and SSL between Cloudflare and your web server. You'll need to have your own SSL cert or self-signed cert at the very least. Full (Strict): SSL between the visitor and Cloudflare -- visitor sees HTTPS on your site, and SSL between Cloudflare and your web server. You'll need to have a valid SSL certificate installed on your web server. This certificate must be signed by a certificate authority, have an expiration date in the future, and respond for the request domain name (hostname).
- [ ] [Change TLS Client Auth setting](https://api.cloudflare.com/#zone-settings-change-tls-client-auth-setting) - TLS Client Auth requires Cloudflare to connect to your origin server using a client certificate (Enterprise Only)
- [ ] [Change True Client IP setting](https://api.cloudflare.com/#zone-settings-change-true-client-ip-setting) - Allows customer to continue to use True Client IP (Akamai feature) in the headers we send to the origin. This is limited to Enterprise Zones.
- [ ] [Change Minimum TLS Version setting](https://api.cloudflare.com/#zone-settings-change-minimum-tls-version-setting) - Only accept HTTPS requests that use at least the TLS protocol version specified. For example, if TLS 1.1 is selected, TLS 1.0 connections will be rejected, while 1.1, 1.2, and 1.3 (if enabled) will be permitted.
- [ ] [Change Ciphers setting](https://api.cloudflare.com/#zone-settings-change-ciphers-setting) - A whitelist of ciphers for TLS termination. These ciphers must be in the BoringSSL format.
- [ ] [Change TLS 1.3 setting](https://api.cloudflare.com/#zone-settings-change-tls-1.3-setting) - Enable Crypto TLS 1.3 feature for this zone.
- [ ] [Change Web Application Firewall (WAF) setting](https://api.cloudflare.com/#zone-settings-change-web-application-firewall-waf-setting) - The WAF examines HTTP requests to your website. It inspects both GET and POST requests and applies rules to help filter out illegitimate traffic from legitimate website visitors. The Cloudflare WAF inspects website addresses or URLs to detect anything out of the ordinary. If the Cloudflare WAF determines suspicious user behavior, then the WAF will 'challenge' the web visitor with a page that asks them to submit a CAPTCHA successfully to continue their action. If the challenge is failed, the action will be stopped. What this means is that Cloudflare's WAF will block any traffic identified as illegitimate before it reaches your origin web server.
- [ ] [Change HTTP2 setting](https://api.cloudflare.com/#zone-settings-change-http2-setting) - Value of the HTTP2 setting
- [ ] [Change HTTP3 setting](https://api.cloudflare.com/#zone-settings-change-http3-setting) - Value of the HTTP3 setting
- [ ] [Change 0-RTT session resumption setting](https://api.cloudflare.com/#zone-settings-change-0-rtt-session-resumption-setting) - Value of the 0-RTT setting
- [ ] [Change Pseudo IPv4 setting](https://api.cloudflare.com/#zone-settings-change-pseudo-ipv4-setting) - Value of the Pseudo IPv4 setting
- [ ] [Change WebSockets setting](https://api.cloudflare.com/#zone-settings-change-websockets-setting) - WebSockets are open connections sustained between the client and the origin server. Inside a WebSockets connection, the client and the origin can pass data back and forth without having to reestablish sessions. This makes exchanging data within a WebSockets connection fast. WebSockets are often used for real-time applications such as live chat and gaming. 
- [ ] [Change Image Resizing setting](https://api.cloudflare.com/#zone-settings-change-image-resizing-setting) - Image Resizing provides on-demand resizing, conversion and optimisation for images served through Cloudflare's network.
- [ ] [Change HTTP/2 Edge Prioritization setting](https://api.cloudflare.com/#zone-settings-change-http/2-edge-prioritization-setting) - HTTP/2 Edge Prioritization optimises the delivery of resources served through HTTP/2 to improve page load performance. It also supports fine control of content delivery when used in conjunction with Workers.

[Zone Analytics](https://api.cloudflare.com/#zone-analytics-properties) **(Deprecated from 31-MAY-2020)** - Analytics data for a zone. Please use the new GraphQL Analytics API instead: https://developers.cloudflare.com/analytics/graphql-api/. It provides equivalent data and more features, including the ability to select only the metrics you need. Migration guide: https://developers.cloudflare.com/analytics/migration-guides/zone-analytics/.
- [ ] [Dashboard](https://api.cloudflare.com/#zone-analytics-dashboard) - The dashboard view provides both totals and timeseries data for the given zone and time period across the entire Cloudflare network.
- [ ] [Analytics by Co-locations](https://api.cloudflare.com/#zone-analytics-analytics-by-co-locations) - This view provides a breakdown of analytics data by datacenter. Note: This is available to Enterprise customers only.

[Logs Received](https://api.cloudflare.com/#logs-received-properties) - Edge http logs received
- [ ] [Get log retention flag](https://api.cloudflare.com/#logs-received-get-log-retention-flag) - Get log retention flag for Logpull API
- [ ] [Update log retention flag](https://api.cloudflare.com/#logs-received-update-log-retention-flag) - Update log retention flag for Logpull API
- [ ] [Logs Received](https://api.cloudflare.com/#logs-received-logs-received) - The /received api route allows customers to retrieve their edge http logs. The basic access pattern is "give me all the logs for zone Z for minute M" where the minute M refers to the time records were received at Cloudflare's central data center. start is inclusive, and end is exclusive. Because of that, to get all data, at minutely cadence, starting at 10AM, the proper values are: start=2018-05-20T10:00:00Z&end=2018-05-20T10:01:00Z, then start=2018-05-20T10:01:00Z&end=2018-05-20T10:02:00Z and so on; the "overlap" will be handled properly.
- [ ] [Logs RayIDs](https://api.cloudflare.com/#logs-received-logs-rayids) - The /rayids api route allows lookups by specific rayid. The rayids route will return 0, 1, or more records (ray ids are not unique).
- [ ] [Fields](https://api.cloudflare.com/#logs-received-fields) - The list of all fields available. The response is json object with key-value pairs where keys are field names, and values are descriptions.

[Logpush Jobs](https://api.cloudflare.com/#logpush-jobs-properties) - Logpush Jobs
- [ ] [List Logpush Jobs](https://api.cloudflare.com/#logpush-jobs-list-logpush-jobs) - List Logpush Jobs for a zone
- [ ] [List Logpush Jobs for a dataset](https://api.cloudflare.com/#logpush-jobs-list-logpush-jobs-for-a-dataset) - List Logpush Jobs for a zone for a dataset
- [ ] [Fields](https://api.cloudflare.com/#logpush-jobs-fields) - The list of all fields available for a dataset. The response result is an object with key-value pairs where keys are field names, and values are descriptions.
- [ ] [Get Ownership Challenge](https://api.cloudflare.com/#logpush-jobs-get-ownership-challenge) - Get a new ownership challenge sent to your destination
- [ ] [Validate Ownership Challenge](https://api.cloudflare.com/#logpush-jobs-validate-ownership-challenge) - Validate ownership challenge of the destination
- [ ] [Validate Origin](https://api.cloudflare.com/#logpush-jobs-validate-origin) - Validate logpull origin with logpull_options
- [ ] [Create Logpush Job](https://api.cloudflare.com/#logpush-jobs-create-logpush-job) - Create a new Logpush Job for a zone
- [ ] [Logpush Job Details](https://api.cloudflare.com/#logpush-jobs-logpush-job-details) - Get the details of a Logpush Job
- [ ] [Update Logpush Job](https://api.cloudflare.com/#logpush-jobs-update-logpush-job) - Update a Logpush Job
- [ ] [Delete Logpush Job](https://api.cloudflare.com/#logpush-jobs-delete-logpush-job) - Delete a Logpush Job
- [ ] [Check Destination Exists](https://api.cloudflare.com/#logpush-jobs-check-destination-exists) - Check if there is an existing job with a destination

[DNS Records for a Zone](https://api.cloudflare.com/#dns-records-for-a-zone-properties) - Documentation for Cloudflare DNS records
- [X] [List DNS Records](https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records) - List, search, sort, and filter a zones' DNS records.
- [X] [Create DNS Record](https://api.cloudflare.com/#dns-records-for-a-zone-create-dns-record) - Create a new DNS record for a zone. See the record object definitions for required attributes for each record type
- [ ] [DNS Record Details](https://api.cloudflare.com/#dns-records-for-a-zone-dns-record-details) - Get DNS Record Details
- [ ] [Update DNS Record](https://api.cloudflare.com/#dns-records-for-a-zone-update-dns-record)
- [ ] [Patch DNS Record](https://api.cloudflare.com/#dns-records-for-a-zone-patch-dns-record)
- [X] [Delete DNS Record](https://api.cloudflare.com/#dns-records-for-a-zone-delete-dns-record)
- [ ] [Import DNS Records](https://api.cloudflare.com/#dns-records-for-a-zone-import-dns-records) - You can upload your BIND config through this endpoint. It assumes that cURL is called from a location with bind_config.txt (valid BIND config) present.
- [ ] [Export DNS Records](https://api.cloudflare.com/#dns-records-for-a-zone-export-dns-records) - You can export your BIND config through this endpoint.

[DNS Analytics](https://api.cloudflare.com/#dns-analytics-properties) - Analytics data for a zone.
- [ ] [Table](https://api.cloudflare.com/#dns-analytics-table) - Retrieves a list of summarised aggregate metrics over a given time period.
- [ ] [By Time](https://api.cloudflare.com/#dns-analytics-by-time) - Retrieves a list of aggregate metrics grouped by time interval.

[DNS Firewall (Organizations)](https://api.cloudflare.com/#dns-firewall-organizations--properties) **(Deprecated from 04-FEB-2020)** - Organization-level DNS Firewall Management
- [ ] [List DNS Firewall Clusters](https://api.cloudflare.com/#dns-firewall-organizations--list-dns-firewall-clusters) - List configured DNS Firewall clusters for a user
- [ ] [Create DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-organizations--create-dns-firewall-cluster) - Create a configured DNS Firewall Cluster
- [ ] [DNS Firewall Cluster Details](https://api.cloudflare.com/#dns-firewall-organizations--dns-firewall-cluster-details) - List a single configured DNS Firewall clusters for a user
- [ ] [Edit DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-organizations--edit-dns-firewall-cluster) - Edit a DNS Firewall Cluster configuration
- [ ] [Delete DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-organizations--delete-dns-firewall-cluster) - Delete a configured DNS Firewall Cluster

[DNS Firewall (Accounts)](https://api.cloudflare.com/#dns-firewall-accounts--properties) - Account-level DNS Firewall Management
- [ ] [List DNS Firewall Clusters](https://api.cloudflare.com/#dns-firewall-accounts--list-dns-firewall-clusters) - List configured DNS Firewall clusters for an account
- [ ] [DNS Firewall Cluster Details](https://api.cloudflare.com/#dns-firewall-accounts--dns-firewall-cluster-details) - List a single configured DNS Firewall clusters for an account
- [ ] [Update DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-accounts--update-dns-firewall-cluster) - Modify a DNS Firewall Cluster configuration
- [ ] [Delete DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-accounts--delete-dns-firewall-cluster) - Delete a configured DNS Firewall Cluster
- [ ] [Create DNS Firewall Cluster](https://api.cloudflare.com/#dns-firewall-accounts--create-dns-firewall-cluster) - Create a configured DNS Firewall Cluster

[DNS Firewall Analytics (Organizations)](https://api.cloudflare.com/#dns-firewall-analytics-organizations--properties) **(Deprecated from 04-FEB-2020)** - Analytics data for a DNS Firewall cluster
- [ ] [Table](https://api.cloudflare.com/#dns-firewall-analytics-organizations--table) - Retrieves a list of summarised aggregate metrics over a given time period.
- [ ] [By Time](https://api.cloudflare.com/#dns-firewall-analytics-organizations--by-time) - Retrieves a list of aggregate metrics grouped by time interval.

[DNS Firewall Analytics (Accounts)](https://api.cloudflare.com/#dns-firewall-analytics-accounts--properties) - Analytics data for a DNS Firewall cluster.
- [ ] [Table](https://api.cloudflare.com/#dns-firewall-analytics-accounts--table) - Retrieves a list of summarised aggregate metrics over a given time period.
- [ ] [By Time](https://api.cloudflare.com/#dns-firewall-analytics-accounts--by-time) - Retrieves a list of aggregate metrics grouped by time interval.

[Secondary DNS](https://api.cloudflare.com/#secondary-dns-properties) - Secondary DNS Zone
- [ ] [Secondary Zone Configuration Details](https://api.cloudflare.com/#secondary-dns-secondary-zone-configuration-details) - Get secondary zone configuration
- [ ] [Update Secondary Zone Configuration](https://api.cloudflare.com/#secondary-dns-update-secondary-zone-configuration) - Update secondary zone configuration
- [ ] [Delete Secondary Zone Configuration](https://api.cloudflare.com/#secondary-dns-delete-secondary-zone-configuration) - Delete secondary zone configuration
- [ ] [Create Secondary Zone Configuration](https://api.cloudflare.com/#secondary-dns-create-secondary-zone-configuration) - Create secondary zone configuration
- [ ] [Force Secondary Zone AXFR](https://api.cloudflare.com/#secondary-dns-force-secondary-zone-axfr) - Force Secondary Zone AXFR

[Secondary DNS (TSIG)](https://api.cloudflare.com/#secondary-dns-tsig--properties) - TSIG key management for secondary DNS
- [ ] [List TSIGs](https://api.cloudflare.com/#secondary-dns-tsig--list-tsigs) - List TSIGs
- [ ] [TSIG Details](https://api.cloudflare.com/#secondary-dns-tsig--tsig-details) - Get TSIG
- [ ] [Update TSIG](https://api.cloudflare.com/#secondary-dns-tsig--update-tsig) - Modify TSIG
- [ ] [Delete TSIG](https://api.cloudflare.com/#secondary-dns-tsig--delete-tsig) - Delete TSIG
- [ ] [Create TSIG](https://api.cloudflare.com/#secondary-dns-tsig--create-tsig) - Create TSIG

[Secondary DNS (Master)](https://api.cloudflare.com/#secondary-dns-master--properties) - Master nameservers for secondary DNS
- [ ] [List Masters](https://api.cloudflare.com/#secondary-dns-master--list-masters) - List Masters
- [ ] [Master Details](https://api.cloudflare.com/#secondary-dns-master--master-details) - Get Master
- [ ] [Update Master](https://api.cloudflare.com/#secondary-dns-master--update-master) - Modify Master
- [ ] [Delete Master](https://api.cloudflare.com/#secondary-dns-master--delete-master) - Delete Master
- [ ] [Create Master](https://api.cloudflare.com/#secondary-dns-master--create-master) - Create Master

[Cloudflare IPs](https://api.cloudflare.com/#cloudflare-ips-properties) - Cloudflare IP space
- [ ] [Cloudflare IP Details](https://api.cloudflare.com/#cloudflare-ips-cloudflare-ip-details) - Get Cloudflare IPs

[Custom Pages for a Zone](https://api.cloudflare.com/#custom-pages-for-a-zone-properties) - Custom pages associated with a zone
- [ ] [List Available Custom Pages](https://api.cloudflare.com/#custom-pages-for-a-zone-list-available-custom-pages) - A list of available Custom Pages the zone can use
- [ ] [Custom Page Details](https://api.cloudflare.com/#custom-pages-for-a-zone-custom-page-details) - Details about a specific Custom page details
- [ ] [Update Custom Page URL](https://api.cloudflare.com/#custom-pages-for-a-zone-update-custom-page-url) - Update custom page URL

[Custom SSL for a Zone](https://api.cloudflare.com/#custom-ssl-for-a-zone-properties) - Custom SSL certificate for a zone
- [ ] [List SSL Configurations](https://api.cloudflare.com/#custom-ssl-for-a-zone-list-ssl-configurations) - List, search, and filter all of your custom SSL certificates. The higher priority will break ties across overlapping 'legacy_custom' certificates, but 'legacy_custom' certificates will always supercede 'sni_custom' certificates.
- [ ] [Create SSL Configuration](https://api.cloudflare.com/#custom-ssl-for-a-zone-create-ssl-configuration) - Upload a new SSL certificate for a zone
- [ ] [SSL Configuration Details](https://api.cloudflare.com/#custom-ssl-for-a-zone-ssl-configuration-details)
- [ ] [Edit SSL Configuration](https://api.cloudflare.com/#custom-ssl-for-a-zone-edit-ssl-configuration) - Upload a new private key and/or PEM/CRT for the SSL certificate. Note: PATCHing a configuration for sni_custom certificates will result in a new resource id being returned, and the previous one being deleted.
- [ ] [Delete SSL Configuration](https://api.cloudflare.com/#custom-ssl-for-a-zone-delete-ssl-configuration) - Remove a SSL certificate from a zone.
- [ ] [Re-prioritize SSL Certificates](https://api.cloudflare.com/#custom-ssl-for-a-zone-re-prioritize-ssl-certificates) - If a zone has multiple SSL certificates, you can set the order in which they should be used during a request. The higher priority will break ties across overlapping 'legacy_custom' certificates.

[Custom Hostname for a Zone](https://api.cloudflare.com/#custom-hostname-for-a-zone-properties) - Manage hostnames for your zone that are routed via CNAME.
- [ ] [List Custom Hostnames](https://api.cloudflare.com/#custom-hostname-for-a-zone-list-custom-hostnames) - List, search, sort, and filter all of your custom hostnames
- [ ] [Create Custom Hostname](https://api.cloudflare.com/#custom-hostname-for-a-zone-create-custom-hostname) - Add a new custom hostname and request that an SSL certificate be issued for it. One of three validation methods—http, cname, email—should be used, with 'http' recommended if the CNAME is already in place (or will be soon). Specifying 'email' will send an email to the WHOIS contacts on file for the base domain plus hostmaster, postmaster, webmaster, admin, administrator. Specifying 'cname' will return a CNAME that needs to be placed. If http is used and the domain is not already pointing to the Managed CNAME host, the PATCH method must be used once it is (to complete validation).
- [ ] [Custom Hostname Details](https://api.cloudflare.com/#custom-hostname-for-a-zone-custom-hostname-details)
- [ ] [Edit Custom Hostname](https://api.cloudflare.com/#custom-hostname-for-a-zone-edit-custom-hostname) - Modify SSL configuration for a custom hostname. When sent with SSL config that matches existing config, used to indicate that hostname should pass domain control validation (DCV). Can also be used to change validation type, e.g., from 'http' to 'email'.
- [ ] [Delete Custom Hostname (and any issued SSL certificates)](https://api.cloudflare.com/#custom-hostname-for-a-zone-delete-custom-hostname-and-any-issued-ssl-certificates-)

[Keyless SSL for a Zone](https://api.cloudflare.com/#keyless-ssl-for-a-zone-properties) - A Keyless certificate is an SSL certificate where the SSL private key is not stored on Cloudflare
- [ ] [List Keyless SSL Configurations](https://api.cloudflare.com/#keyless-ssl-for-a-zone-list-keyless-ssl-configurations) - List all Keyless SSL configurations for a given zone
- [ ] [Create Keyless SSL Configuration](https://api.cloudflare.com/#keyless-ssl-for-a-zone-create-keyless-ssl-configuration)
- [ ] [Get Keyless SSL Configuration](https://api.cloudflare.com/#keyless-ssl-for-a-zone-get-keyless-ssl-configuration) - Get details for one Keyless SSL configuration
- [ ] [Edit Keyless SSL Configuration](https://api.cloudflare.com/#keyless-ssl-for-a-zone-edit-keyless-ssl-configuration) - This will update attributes of a Keyless SSL. Consists of one or more of the following: host,name,port,certificate,enabled
- [ ] [Delete Keyless SSL Configuration](https://api.cloudflare.com/#keyless-ssl-for-a-zone-delete-keyless-ssl-configuration)

[Analyze Certificate](https://api.cloudflare.com/#analyze-certificate-properties)
- [ ] [Analyze Certificate](https://api.cloudflare.com/#analyze-certificate-analyze-certificate) - Returns the set of hostnames, the signature algorithm, and the expiration date of the certificate.

[Certificate Packs](https://api.cloudflare.com/#certificate-packs-properties)
- [ ] [List Certificate Packs](https://api.cloudflare.com/#certificate-packs-list-certificate-packs) - For a given zone, list all certificate packs
- [ ] [Order Certificate Pack](https://api.cloudflare.com/#certificate-packs-order-certificate-pack) - For a given zone, order a certificate pack with a list of hostnames

[SSL Verification](https://api.cloudflare.com/#ssl-verification-properties) - SSL Verification for a Zone
- [ ] [SSL Verification Details](https://api.cloudflare.com/#ssl-verification-ssl-verification-details) - Get SSL Verification Info for a Zone
- [ ] [Edit SSL Certificate Pack Validation Method](https://api.cloudflare.com/#ssl-verification-edit-ssl-certificate-pack-validation-method) - Edit SSL validation method for a certificate pack. A PATCH request will request an immediate validation check on any certificate, and return the updated status. If a validation method is provided, the validation will be immediately attempted using that method.

[Universal SSL Settings for a Zone](https://api.cloudflare.com/#universal-ssl-settings-for-a-zone-properties) - Universal SSL Settings for a Zone
- [ ] [Universal SSL Settings Details](https://api.cloudflare.com/#universal-ssl-settings-for-a-zone-universal-ssl-settings-details) - Get Universal SSL Settings for a Zone
- [ ] [Edit Universal SSL Settings](https://api.cloudflare.com/#universal-ssl-settings-for-a-zone-edit-universal-ssl-settings) - Patch Universal SSL Settings for a Zone

[Origin CA](https://api.cloudflare.com/#origin-ca-properties) - API to create Cloudflare-issued SSL certificates that can be installed on your origin server. Use your Origin CA Key as your User Service Key when calling these endpoints (see the section on request headers for details).
- [ ] [List Certificates](https://api.cloudflare.com/#origin-ca-list-certificates) - List all existing Origin CA certificates for a given zone. Use your Origin CA Key as your User Service Key when calling this endpoint
- [ ] [Create Certificate](https://api.cloudflare.com/#origin-ca-create-certificate) - Create an Origin CA certificate. Use your Origin CA Key as your User Service Key when calling this endpoint
- [ ] [Get Certificate](https://api.cloudflare.com/#origin-ca-get-certificate) - Get an existing Origin CA certificate by its serial number. Use your Origin CA Key as your User Service Key when calling this endpoint
- [ ] [Revoke Certificate](https://api.cloudflare.com/#origin-ca-revoke-certificate) - Revoke an existing Origin CA certificate by its serial number. Use your Origin CA Key as your User Service Key when calling this endpoint

[Stream Videos](https://api.cloudflare.com/#stream-videos-properties) - You can upload videos to Cloudflare Stream for fast video processing and delivery.
- [ ] [List Videos](https://api.cloudflare.com/#stream-videos-list-videos)
- [ ] [Initiate a Video Upload](https://api.cloudflare.com/#stream-videos-initiate-a-video-upload) - Initiate a video upload using the TUS protocol. On success, server will response with status code 201 (Created) and include a 'location' header indicating where the video content should be uploaded to. (See https://tus.io for protocol details.). On success the body of the response will be empty (zero-length content). On failure the body will contain a JSON error response.
- [ ] [Upload a Video Content Range](https://api.cloudflare.com/#stream-videos-upload-a-video-content-range) - Upload a portion of a video using the TUS protocol (See https://tus.io for protocol details). On success return 204 (No Content) with no body content. Each chunk, except the final chunk must be 5242880 bytes in size for uploads larger than 5242880 bytes.
- [ ] [Video Details](https://api.cloudflare.com/#stream-videos-video-details) - Fetch details of a single video.
- [ ] [Embed Code HTML](https://api.cloudflare.com/#stream-videos-embed-code-html) - Fetch an HTML code snippet that can be used to embed the video in a web page that is delivered through Cloudflare. On success returns an HTML fragment (not a full document) that can be included on a web page to display the video. On failure returns a JSON response body (see Error Response) above.
- [ ] [Link to Video](https://api.cloudflare.com/#stream-videos-link-to-video) - Receive an HTTP redirect response to an HTML page containing a preview of this video. On success returns status code 301 and a location header indicating the new location, with no body. On failure returns a JSON response body (see Error Response above).
- [ ] [Delete video](https://api.cloudflare.com/#stream-videos-delete-video) - Delete a video on Cloudflare Stream. On success, all copies of the video are deleted.

[Worker Script](https://api.cloudflare.com/#worker-script-properties) - A Worker script is a single script that is executed on matching routes in the Cloudflare edge
- [ ] [Upload Worker](https://api.cloudflare.com/#worker-script-upload-worker) - Upload a worker, or a new version of a worker.
- [ ] [List Workers](https://api.cloudflare.com/#worker-script-list-workers) - Fetch a list of uploaded workers.
- [ ] [Download Worker](https://api.cloudflare.com/#worker-script-download-worker) - Fetch raw script content for your worker. Note this is the original script content, not JSON encoded.
- [ ] [Delete Worker](https://api.cloudflare.com/#worker-script-delete-worker) - Delete your worker. This call has no response body on a successful delete.

[Worker Routes](https://api.cloudflare.com/#worker-routes-properties) - Routes are basic patterns used to enable or disable workers that match requests.
- [X] [Create Route](https://api.cloudflare.com/#worker-routes-create-route)
- [X] [List Routes](https://api.cloudflare.com/#worker-routes-list-routes)
- [ ] [Get Route](https://api.cloudflare.com/#worker-routes-get-route)
- [ ] [Update Route](https://api.cloudflare.com/#worker-routes-update-route)
- [X] [Delete Route](https://api.cloudflare.com/#worker-routes-delete-route)

[Workers KV Namespace](https://api.cloudflare.com/#workers-kv-namespace-properties) - A Namespace is a collection of key-value pairs stored in Workers KV.
- [X] [List Namespaces](https://api.cloudflare.com/#workers-kv-namespace-list-namespaces) - Returns the namespaces owned by an account
- [X] [Create a Namespace](https://api.cloudflare.com/#workers-kv-namespace-create-a-namespace) - Creates a namespace under the given title. A 400 is returned if the account already owns a namespace with this title. A namespace must be explicitly deleted to be replaced.
- [X] [Remove a Namespace](https://api.cloudflare.com/#workers-kv-namespace-remove-a-namespace) - Deletes the namespace corresponding to the given ID.
- [X] [Rename a Namespace](https://api.cloudflare.com/#workers-kv-namespace-rename-a-namespace) - Modifies a namespace's title.
- [X] [List a Namespace's Keys](https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys) - Lists a namespace's keys.
- [ ] [Read key-value pair](https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair) - Returns the value associated with the given key in the given namespace. Use URL-encoding to use special characters (e.g. :, !, %) in the key name. If the KV-pair is set to expire at some point, the expiration time as measured in seconds since the UNIX epoch will be returned in the "Expiration" response header.
- [ ] [Write key-value pair](https://api.cloudflare.com/#workers-kv-namespace-write-key-value-pair) - Write a value identified by a key. Use URL-encoding to use special characters (e.g. :, !, %) in the key name. Body should be the value to be stored. Existing values and expirations will be overwritten. If neither expiration nor expiration_ttl is specified, the key-value pair will never expire. If both are set, expiration_ttl is used and expiration is ignored.
- [X] [Write multiple key-value pairs](https://api.cloudflare.com/#workers-kv-namespace-write-multiple-key-value-pairs) - Write multiple keys and values at once. Body should be an array of up to 10,000 key-value pairs to be stored, along with optional expiration information. Existing values and expirations will be overwritten. If neither expiration nor expiration_ttl is specified, the key-value pair will never expire. If both are set, expiration_ttl is used and expiration is ignored. The entire request size must be 100 megabytes or less.
- [X] [Delete key-value pair](https://api.cloudflare.com/#workers-kv-namespace-delete-key-value-pair) - Remove a KV pair from the Namespace. Use URL-encoding to use special characters (e.g. :, !, %) in the key name.
- [X] [Delete multiple key-value pairs](https://api.cloudflare.com/#workers-kv-namespace-delete-multiple-key-value-pairs) - Remove multiple KV pairs from the Namespace. Body should be an array of up to 10,000 keys to be removed.

[Workers KV Request Analytics](https://api.cloudflare.com/#workers-kv-request-analytics-properties) - Metrics on Workers KV requests.
- [ ] [Query Request Analytics](https://api.cloudflare.com/#workers-kv-request-analytics-query-request-analytics) - Retrieves Workers KV request metrics for the given account.

[Workers KV Stored Data Analytics](https://api.cloudflare.com/#workers-kv-stored-data-analytics-properties) - Metrics on Workers KV stored data.
- [ ] [Query Stored Data Analytics](https://api.cloudflare.com/#workers-kv-stored-data-analytics-query-stored-data-analytics) - Retrieves Workers KV stored data metrics for the given account.

[Spectrum Applications](https://api.cloudflare.com/#spectrum-applications-properties) - You can extend the power of Cloudflare's DDoS, TLS, and IP Firewall to your other TCP-based services.
- [ ] [List Spectrum Applications](https://api.cloudflare.com/#spectrum-applications-list-spectrum-applications) - Retrieve a list of currently existing Spectrum Applications inside a zone.
- [ ] [Create Spectrum Application](https://api.cloudflare.com/#spectrum-applications-create-spectrum-application) - Create a new Spectrum Application from a configuration
- [ ] [Get Spectrum Application Configuration](https://api.cloudflare.com/#spectrum-applications-get-spectrum-application-configuration) - Get the application configuration of a specific application inside a zone.
- [ ] [Update Spectrum Application Configuration](https://api.cloudflare.com/#spectrum-applications-update-spectrum-application-configuration) - Update a previously existing application's configuration.
- [ ] [Delete Spectrum Application](https://api.cloudflare.com/#spectrum-applications-delete-spectrum-application) - Delete a previously existing application.

[Spectrum Analytics (Summary)](https://api.cloudflare.com/#spectrum-analytics-summary--properties) - Summarized analytics data for Spectrum applications.
- [ ] [Get Analytics Summary](https://api.cloudflare.com/#spectrum-analytics-summary--get-analytics-summary) - Retrieves a list of summarised aggregate metrics over a given time period.

[Spectrum Analytics (By Time)](https://api.cloudflare.com/#spectrum-analytics-by-time--properties) - Analytics data for Spectrum applications grouped by time interval.
- [ ] [Get Analytics By Time](https://api.cloudflare.com/#spectrum-analytics-by-time--get-analytics-by-time) - Retrieves a list of aggregate metrics grouped by time interval.

[Spectrum Aggregate Analytics](https://api.cloudflare.com/#spectrum-aggregate-analytics-properties) - Aggregated Analytics for Spectrum in real time.
- [ ] [Get Current Aggregate Analytics](https://api.cloudflare.com/#spectrum-aggregate-analytics-get-current-aggregate-analytics) - Retrieves analytics aggregated from the last minute of usage on Spectrum Applications underneath a given zone.

[Page Rules for a Zone](https://api.cloudflare.com/#page-rules-for-a-zone-properties) - A rule describing target patterns for requests and actions to perform on matching requests
- [ ] [List Page Rules](https://api.cloudflare.com/#page-rules-for-a-zone-list-page-rules)
- [ ] [Create Page Rule](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule)
- [ ] [Page Rule Details](https://api.cloudflare.com/#page-rules-for-a-zone-page-rule-details)
- [ ] [Update Page Rule](https://api.cloudflare.com/#page-rules-for-a-zone-update-page-rule) - Replace a page rule. The final rule will exactly match the data passed with this request.
- [ ] [Edit Page Rule](https://api.cloudflare.com/#page-rules-for-a-zone-edit-page-rule)
- [ ] [Delete Page Rule](https://api.cloudflare.com/#page-rules-for-a-zone-delete-page-rule)

[Available Page Rules for a Zone](https://api.cloudflare.com/#available-page-rules-for-a-zone-properties) - A rule describing target patterns for requests and actions to perform on matching requests
- [ ] [List Available Page rule setting](https://api.cloudflare.com/#available-page-rules-for-a-zone-list-available-page-rule-setting)

[Rate Limits for a Zone](https://api.cloudflare.com/#rate-limits-for-a-zone-properties) - Documentation for Cloudflare Rate Limits
- [ ] [List Rate Limits](https://api.cloudflare.com/#rate-limits-for-a-zone-list-rate-limits) - List the rate limits on a zone.
- [ ] [Create Rate Limit](https://api.cloudflare.com/#rate-limits-for-a-zone-create-rate-limit) - Create a new rate limit for a zone. See the record object definitions for required attributes for each record type
- [ ] [Rate Limit Details](https://api.cloudflare.com/#rate-limits-for-a-zone-rate-limit-details)
- [ ] [Update Rate Limit](https://api.cloudflare.com/#rate-limits-for-a-zone-update-rate-limit)
- [ ] [Delete Rate Limit](https://api.cloudflare.com/#rate-limits-for-a-zone-delete-rate-limit)

[User-level Firewall Access Rule](https://api.cloudflare.com/#user-level-firewall-access-rule-properties) - A firewall access rule applied to all zones owned by the user
- [ ] [List Access Rules](https://api.cloudflare.com/#user-level-firewall-access-rule-list-access-rules) - Search, sort, and filter IP/country access rules
- [ ] [Create Access Rule](https://api.cloudflare.com/#user-level-firewall-access-rule-create-access-rule) - Make a new IP, IP range, or country access rule for all zones owned by the user. Note: If you would like to create an access rule that applies to a specific zone only, use the zone firewall endpoints.
- [ ] [Edit Access Rule](https://api.cloudflare.com/#user-level-firewall-access-rule-edit-access-rule) - Edit rule state and/or note. This will be applied across all zones owned by the user.
- [ ] [Delete Access Rule](https://api.cloudflare.com/#user-level-firewall-access-rule-delete-access-rule) - Remove an access rule so it is no longer evaluated during requests. This will apply to all zones owned by the user.

[Account-level Firewall access rule](https://api.cloudflare.com/#account-level-firewall-access-rule-properties) - A firewall access rule applied to all zones owned by the account
- [ ] [List Access Rules](https://api.cloudflare.com/#account-level-firewall-access-rule-list-access-rules) - Search, sort, and filter IP/country access rules
- [ ] [Create Access Rule](https://api.cloudflare.com/#account-level-firewall-access-rule-create-access-rule) - Make a new IP, IP range, or country access rule for all zones owned by the account. Note: If you would like to create an access rule that applies to a specific zone only, use the zone firewall endpoints.
- [ ] [Access Rule Details](https://api.cloudflare.com/#account-level-firewall-access-rule-access-rule-details) - Get the details of an access rule
- [ ] [Update Access Rule](https://api.cloudflare.com/#account-level-firewall-access-rule-update-access-rule) - Update rule state and/or configuration. This will be applied across all zones owned by the account.
- [ ] [Delete Access Rule](https://api.cloudflare.com/#account-level-firewall-access-rule-delete-access-rule) - Remove an access rule so it is no longer evaluated during requests. This will apply to all zones owned by the account.

[Organization-level Firewall Access Rule](https://api.cloudflare.com/#organization-level-firewall-access-rule-properties) **(Deprecated from 04-FEB-2020)** - A firewall access rule applied to all zones owned by the organization
- [ ] [List Access Rules](https://api.cloudflare.com/#organization-level-firewall-access-rule-list-access-rules) - Search, sort, and filter IP/country access rules
- [ ] [Create Access Rule](https://api.cloudflare.com/#organization-level-firewall-access-rule-create-access-rule) - Make a new IP, IP range, or country access rule for all zones owned by the organization. Note: If you would like to create an access rule that applies to a specific zone only, use the zone firewall endpoints.
- [ ] [Edit Access Rule](https://api.cloudflare.com/#organization-level-firewall-access-rule-edit-access-rule) - Edit rule state and/or note. This will be applied across all zones owned by the organization.
- [ ] [Delete Access Rule](https://api.cloudflare.com/#organization-level-firewall-access-rule-delete-access-rule) - Remove an access rule so it is no longer evaluated during requests. This will apply to all zones owned by the organization.

[Firewall Access Rule for a Zone](https://api.cloudflare.com/#firewall-access-rule-for-a-zone-properties) - An IP, IP range, or country specific firewall rule applied directly to a zone or inherited from user or organization-level rules.
- [ ] [List Access Rules](https://api.cloudflare.com/#firewall-access-rule-for-a-zone-list-access-rules) - Search, sort, and filter IP/country access rules
- [ ] [Create Access Rule](https://api.cloudflare.com/#firewall-access-rule-for-a-zone-create-access-rule) - Make a new IP, IP range, or country access rule for the zone. Note: If you would like to create an access rule that applies across all of your owned zones, use the user or organization firewall endpoints as appropriate.
- [ ] [Edit Access Rule](https://api.cloudflare.com/#firewall-access-rule-for-a-zone-edit-access-rule) - Update rule state and/or note for the zone. Note: you can only edit rules in the 'zone' group via this endpoint. Use the appropriate owner rules endpoint if trying to manage owner-level rules
- [ ] [Delete Access Rule](https://api.cloudflare.com/#firewall-access-rule-for-a-zone-delete-access-rule) - Remove an access rule so it is no longer evaluated during requests. Optionally, specify how to delete rules that match the mode and configuration across all other zones that this zone owner manages. 'none' is the default, and will only delete this rule. 'basic' will delete rules that match the same mode and configuration. 'aggressive' will delete rules that match the same configuration.

[WAF Rule Packages](https://api.cloudflare.com/#waf-rule-packages-properties) - Web application firewall rule package applied to a zone
- [ ] [List Firewall Packages](https://api.cloudflare.com/#waf-rule-packages-list-firewall-packages) - Retrieve firewall packages for a zone
- [ ] [Firewall Package Details](https://api.cloudflare.com/#waf-rule-packages-firewall-package-details) - Get information about a single firewall package
- [ ] [Edit Firewall Package](https://api.cloudflare.com/#waf-rule-packages-edit-firewall-package) - Change the sensitivity and action for an anomaly detection type WAF rule package

[WAF Rule Groups](https://api.cloudflare.com/#waf-rule-groups-properties) - A group of web application firewall rules that share common functionality and traits
- [ ] [List Rule Groups](https://api.cloudflare.com/#waf-rule-groups-list-rule-groups) - Search, list, and sort rule groups contained within a package
- [ ] [Rule Group Details](https://api.cloudflare.com/#waf-rule-groups-rule-group-details) - Get a single rule group
- [ ] [Edit Rule Group](https://api.cloudflare.com/#waf-rule-groups-edit-rule-group) - Update the state of a rule group

[WAF Rules](https://api.cloudflare.com/#waf-rules-properties) - A firewall rule for a zone
- [ ] [List Rules](https://api.cloudflare.com/#waf-rules-list-rules) - Search, sort, and filter rules within a package
- [ ] [Rule Details](https://api.cloudflare.com/#waf-rules-rule-details) - Individual information about a rule
- [ ] [Edit Rule](https://api.cloudflare.com/#waf-rules-edit-rule) - Update the action the rule will perform if triggered on the zone

[WAF Overrides](https://api.cloudflare.com/#waf-overrides-properties) - Overrides for turning WAF packages on or off
- [ ] [List URI-controlled WAF configurations](https://api.cloudflare.com/#waf-overrides-list-uri-controlled-waf-configurations) - List the WAF configurations on a zone.
- [ ] [Create a URI-controlled WAF configuration](https://api.cloudflare.com/#waf-overrides-create-a-uri-controlled-waf-configuration) - Create a new WAF configuration for a zone. See the record object definitions for required attributes for each record type.
- [ ] [URI-controlled WAF configuration details](https://api.cloudflare.com/#waf-overrides-uri-controlled-waf-configuration-details)
- [ ] [Update URI-controlled WAF configuration](https://api.cloudflare.com/#waf-overrides-update-uri-controlled-waf-configuration)
- [ ] [Delete URI-controlled WAF configuration](https://api.cloudflare.com/#waf-overrides-delete-uri-controlled-waf-configuration)

[User-Agent Blocking Rules](https://api.cloudflare.com/#user-agent-blocking-rules-properties) - Perform access control when matching the exact UserAgent reported by the client
- [ ] [List UserAgent Rules](https://api.cloudflare.com/#user-agent-blocking-rules-list-useragent-rules)
- [ ] [Create UserAgent Rule](https://api.cloudflare.com/#user-agent-blocking-rules-create-useragent-rule) - Create a new UserAgent rule for a zone. See the record object definitions for required attributes for each record type
- [ ] [UserAgent Rule Details](https://api.cloudflare.com/#user-agent-blocking-rules-useragent-rule-details) - List one user agent rule
- [ ] [Update UserAgent Rule](https://api.cloudflare.com/#user-agent-blocking-rules-update-useragent-rule)
- [ ] [Delete UserAgent Rule](https://api.cloudflare.com/#user-agent-blocking-rules-delete-useragent-rule)

[Zone Lockdown](https://api.cloudflare.com/#zone-lockdown-properties) - Lock access to URLs in this zone to only permitted addresses or address ranges.
- [ ] [List Lockdown Rules](https://api.cloudflare.com/#zone-lockdown-list-lockdown-rules) - List the lockdown rules on a zone.
- [ ] [Create Lockdown Rule](https://api.cloudflare.com/#zone-lockdown-create-lockdown-rule) - Create a new lockdown rule for a zone. See the record object definitions for required attributes for each record type
- [ ] [Lockdown Rule Details](https://api.cloudflare.com/#zone-lockdown-lockdown-rule-details)
- [ ] [Update Lockdown Rule](https://api.cloudflare.com/#zone-lockdown-update-lockdown-rule)
- [ ] [Delete Lockdown Rule](https://api.cloudflare.com/#zone-lockdown-delete-lockdown-rule)

[Firewall rules](https://api.cloudflare.com/#firewall-rules-properties) - Define Firewall rules using filter expressions for more control over how traffic is matched to the rule.
- [ ] [List of firewall rules](https://api.cloudflare.com/#firewall-rules-list-of-firewall-rules) - List all the firewall rules currently defined.
- [ ] [Get individual Firewall Rule](https://api.cloudflare.com/#firewall-rules-get-individual-firewall-rule) - Retrieve the properties of an individual firewall rule
- [ ] [Create firewall rules](https://api.cloudflare.com/#firewall-rules-create-firewall-rules) - Create new firewall rules. See the record object definitions for required attributes for each record type.
- [ ] [Update firewall rules](https://api.cloudflare.com/#firewall-rules-update-firewall-rules) - Update existing firewall rules. See the record object definitions for required attributes for each record type
- [ ] [Update individual firewall rule](https://api.cloudflare.com/#firewall-rules-update-individual-firewall-rule) - Update an individual existing firewall rule. See the record object definitions for required attributes for each record type
- [ ] [Delete firewall rules](https://api.cloudflare.com/#firewall-rules-delete-firewall-rules) - Delete existing firewall rules.
- [ ] [Delete individual firewall rules](https://api.cloudflare.com/#firewall-rules-delete-individual-firewall-rules) - Delete existing firewall rules.

[Filters](https://api.cloudflare.com/#filters-properties) - Filter expressions that can be referenced across multiple features
- [ ] [List filters](https://api.cloudflare.com/#filters-list-filters) - List all the filters currently defined
- [ ] [List individual filter](https://api.cloudflare.com/#filters-list-individual-filter) - List one filters currently defined
- [ ] [Create Filters](https://api.cloudflare.com/#filters-create-filters) - Create new filters
- [ ] [Update filters](https://api.cloudflare.com/#filters-update-filters) - Update existing filters. See the record object definitions for required attributes for each record type
- [ ] [Update individual filter](https://api.cloudflare.com/#filters-update-individual-filter) - Update an existing filter. See the record object definitions for required attributes for each record type
- [ ] [Delete filters](https://api.cloudflare.com/#filters-delete-filters) - Delete existing filters.
- [ ] [Delete individual filter](https://api.cloudflare.com/#filters-delete-individual-filter) - Delete existing filters.

[Firewall Events](https://api.cloudflare.com/#firewall-events-properties) - Logs of the mitigations performed by Firewall features.
- [ ] [List Events](https://api.cloudflare.com/#firewall-events-list-events) - Search, and filter Firewall events.

[Load Balancer Monitors](https://api.cloudflare.com/#load-balancer-monitors-properties) - User-level Monitor configurations. Monitors define whether we check over HTTP, HTTPS or TCP, the status code(s) we look for, the interval at which we check, timeouts and response body matching.
- [ ] [List Monitors](https://api.cloudflare.com/#load-balancer-monitors-list-monitors) - List configured monitors for a user.
- [ ] [Create Monitor](https://api.cloudflare.com/#load-balancer-monitors-create-monitor) - Create a configured monitor
- [ ] [Monitor Details](https://api.cloudflare.com/#load-balancer-monitors-monitor-details) - List a single configured monitor for a user
- [ ] [Update Monitor](https://api.cloudflare.com/#load-balancer-monitors-update-monitor) - Modify a configured monitor
- [ ] [Delete Monitor](https://api.cloudflare.com/#load-balancer-monitors-delete-monitor) - Delete a configured monitor

[Load Balancer Pools](https://api.cloudflare.com/#load-balancer-pools-properties) - User-level Load Balancer Pools
- [ ] [List Pools](https://api.cloudflare.com/#load-balancer-pools-list-pools) - List configured pools
- [ ] [Create Pool](https://api.cloudflare.com/#load-balancer-pools-create-pool) - Create a new pool
- [ ] [Pool Details](https://api.cloudflare.com/#load-balancer-pools-pool-details) - Fetch a single configured pool
- [ ] [Pool Health Details](https://api.cloudflare.com/#load-balancer-pools-pool-health-details) - Fetch latest healthcheck details for a single pool
- [ ] [Update Pool](https://api.cloudflare.com/#load-balancer-pools-update-pool) - Modify a configured pool
- [ ] [Delete Pool](https://api.cloudflare.com/#load-balancer-pools-delete-pool) - Delete a configured pool

[Load Balancer Healthcheck Events](https://api.cloudflare.com/#load-balancer-healthcheck-events-properties) - User-level Load Balancing Healthcheck Events Log
- [ ] [List Healthcheck Events](https://api.cloudflare.com/#load-balancer-healthcheck-events-list-healthcheck-events) - List origin health changes

[Account Load Balancer Monitors](https://api.cloudflare.com/#account-load-balancer-monitors-properties) - Account-level Monitor configurations. Monitors define whether we check over HTTP, HTTPS or TCP, the status code(s) we look for, the interval at which we check, timeouts and response body matching.
- [ ] [List Monitors](https://api.cloudflare.com/#account-load-balancer-monitors-list-monitors) - List configured monitors for an account
- [ ] [Create Monitor](https://api.cloudflare.com/#account-load-balancer-monitors-create-monitor) - Create a configured monitor
- [ ] [Monitor Details](https://api.cloudflare.com/#account-load-balancer-monitors-monitor-details) - List a single configured monitor for an account
- [ ] [Update Monitor](https://api.cloudflare.com/#account-load-balancer-monitors-update-monitor) - Modify a configured monitor
- [ ] [Delete Monitor](https://api.cloudflare.com/#account-load-balancer-monitors-delete-monitor) - Delete a configured monitor

[Account Load Balancer Pools](https://api.cloudflare.com/#account-load-balancer-pools-properties) - Account-level Load Balancer Pools
- [ ] [List Pools](https://api.cloudflare.com/#account-load-balancer-pools-list-pools) - List configured pools
- [X] [Create Pool](https://api.cloudflare.com/#account-load-balancer-pools-create-pool) - Create a new pool
- [X] [Pool Details](https://api.cloudflare.com/#account-load-balancer-pools-pool-details) - Fetch a single configured pool
- [ ] [Pool Health Details](https://api.cloudflare.com/#account-load-balancer-pools-pool-health-details) - Fetch latest healthcheck details for a single pool
- [ ] [Update Pool](https://api.cloudflare.com/#account-load-balancer-pools-update-pool) - Modify a configured pool
- [X] [Delete Pool](https://api.cloudflare.com/#account-load-balancer-pools-delete-pool) - Delete a configured pool

[Organization Load Balancer Monitors](https://api.cloudflare.com/#organization-load-balancer-monitors-properties) - Organization-level Monitor configurations. Monitors define whether we check over HTTP, HTTPS or TCP, the status code(s) we look for, the interval at which we check, timeouts and response body matching.
- [ ] [List Monitors](https://api.cloudflare.com/#organization-load-balancer-monitors-list-monitors) - List configured monitors for an organization
- [ ] [Create Monitor](https://api.cloudflare.com/#organization-load-balancer-monitors-create-monitor) - Create a configured monitor
- [ ] [Monitor Details](https://api.cloudflare.com/#organization-load-balancer-monitors-monitor-details) - List a single configured monitor for an organization
- [ ] [Update Monitor](https://api.cloudflare.com/#organization-load-balancer-monitors-update-monitor) - Modify a configured monitor
- [ ] [Delete Monitor](https://api.cloudflare.com/#organization-load-balancer-monitors-delete-monitor) - Delete a configured monitor

[Organization Load Balancer Pools](https://api.cloudflare.com/#organization-load-balancer-pools-properties) - Organization-level Load Balancer Pools
- [ ] [List Pools](https://api.cloudflare.com/#organization-load-balancer-pools-list-pools) - List configured pools
- [ ] [Create Pool](https://api.cloudflare.com/#organization-load-balancer-pools-create-pool) - Create a new pool
- [ ] [Pool Details](https://api.cloudflare.com/#organization-load-balancer-pools-pool-details) - Fetch a single configured pool
- [ ] [Pool Health Details](https://api.cloudflare.com/#organization-load-balancer-pools-pool-health-details) - Fetch latest healthcheck details for a single pool
- [ ] [Update Pool](https://api.cloudflare.com/#organization-load-balancer-pools-update-pool) - Modify a configured pool
- [ ] [Delete Pool](https://api.cloudflare.com/#organization-load-balancer-pools-delete-pool) - Delete a configured pool

[Load Balancers](https://api.cloudflare.com/#load-balancers-properties) - Zone-level Load Balancers
- [ ] [List Load Balancers](https://api.cloudflare.com/#load-balancers-list-load-balancers) - List configured load balancers
- [ ] [Create Load Balancer](https://api.cloudflare.com/#load-balancers-create-load-balancer) - Create a new load balancer
- [ ] [Load Balancer Details](https://api.cloudflare.com/#load-balancers-load-balancer-details) - Fetch a single configured load balancer
- [ ] [Update Load Balancer](https://api.cloudflare.com/#load-balancers-update-load-balancer) - Update a configured load balancer
- [ ] [Delete Load Balancer](https://api.cloudflare.com/#load-balancers-delete-load-balancer) - Delete a configured load balancer

[Railgun](https://api.cloudflare.com/#railgun-properties) - Cloudflare Railgun
- [ ] [List Railguns](https://api.cloudflare.com/#railgun-list-railguns) - List, search, sort and filter your Railguns
- [ ] [Create Railgun](https://api.cloudflare.com/#railgun-create-railgun)
- [ ] [Railgun Details](https://api.cloudflare.com/#railgun-railgun-details)
- [ ] [Enable or Disable a Railgun](https://api.cloudflare.com/#railgun-enable-or-disable-a-railgun) - Enable or disable a Railgun for all zones connected to it
- [ ] [Delete Railgun](https://api.cloudflare.com/#railgun-delete-railgun) - Disable and delete a Railgun. This will immediately disable the Railgun for any connected zones
- [ ] [List Railgun Zones](https://api.cloudflare.com/#railgun-list-railgun-zones) - The zones that are currently using this Railgun

[Railgun Connections for a Zone](https://api.cloudflare.com/#railgun-connections-for-a-zone-properties) - Railguns associated with a zone
- [ ] [List Available Railguns](https://api.cloudflare.com/#railgun-connections-for-a-zone-list-available-railguns) - A list of available Railguns the zone can use
- [ ] [Railgun Details](https://api.cloudflare.com/#railgun-connections-for-a-zone-railgun-details) - Details about a specific Railgun
- [ ] [Connect or Disconnect a Railgun](https://api.cloudflare.com/#railgun-connections-for-a-zone-connect-or-disconnect-a-railgun) - Connect or disconnect a Railgun
- [ ] [Test Railgun Connection](https://api.cloudflare.com/#railgun-connections-for-a-zone-test-railgun-connection) - Test Railgun connection to the Zone

[Account Railguns](https://api.cloudflare.com/#account-railguns-properties) - Cloudflare Railguns available to Accounts
- [ ] [List Railguns](https://api.cloudflare.com/#account-railguns-list-railguns) - List, search, sort and filter your Railguns
- [ ] [Create Railgun](https://api.cloudflare.com/#account-railguns-create-railgun)
- [ ] [Railgun Details](https://api.cloudflare.com/#account-railguns-railgun-details)
- [ ] [Update Railgun](https://api.cloudflare.com/#account-railguns-update-railgun) - Update a Railgun
- [ ] [Delete Railgun](https://api.cloudflare.com/#account-railguns-delete-railgun) - Disable and delete a Railgun. This will immediately disable the Railgun for any connected zones

[Railgun Connections](https://api.cloudflare.com/#railgun-connections-properties) - A Railgun connection associates a zone with the Railgun
- [ ] [List Connections](https://api.cloudflare.com/#railgun-connections-list-connections) - List connections associated with the Railgun
- [ ] [Connection Details](https://api.cloudflare.com/#railgun-connections-connection-details) - Get a connection by ID
- [ ] [Update Connection](https://api.cloudflare.com/#railgun-connections-update-connection) - Enable or disable a connection
- [ ] [Delete Connection](https://api.cloudflare.com/#railgun-connections-delete-connection) - Disable and remove the connection to a zone
- [ ] [Create Connection](https://api.cloudflare.com/#railgun-connections-create-connection) - Associates a zone to the Railgun

[Organization Railgun](https://api.cloudflare.com/#organization-railgun-properties) **(Deprecated from 04-FEB-2020)** - Cloudflare Railgun for Organizations
- [ ] [List Railguns](https://api.cloudflare.com/#organization-railgun-list-railguns) - List, search, sort and filter your Railguns
- [ ] [Create Railgun](https://api.cloudflare.com/#organization-railgun-create-railgun)
- [ ] [Railgun Details](https://api.cloudflare.com/#organization-railgun-railgun-details)
- [ ] [Enable or Disable a Railgun](https://api.cloudflare.com/#organization-railgun-enable-or-disable-a-railgun) - Enable or disable a Railgun for all zones connected to it
- [ ] [Delete Railgun](https://api.cloudflare.com/#organization-railgun-delete-railgun) - Disable and delete a Railgun. This will immediately disable the Railgun for any connected zones
- [ ] [Get Railgun Zones](https://api.cloudflare.com/#organization-railgun-get-railgun-zones) - The zones that are currently using this Railgun

[AML](https://api.cloudflare.com/#aml-properties) - Accelerated Mobile Links
- [ ] [AML Viewer Details](https://api.cloudflare.com/#aml-aml-viewer-details) - Fetch AML configuration for a zone
- [ ] [Update AML Viewer](https://api.cloudflare.com/#aml-update-aml-viewer) - Update AML configuration for a zone

[Custom Pages (Account)](https://api.cloudflare.com/#custom-pages-account--properties) - Custom pages at the account level
- [ ] [List Custom Pages](https://api.cloudflare.com/#custom-pages-account--list-custom-pages) - A list of available account-level Custom Pages
- [ ] [Custom Page Details](https://api.cloudflare.com/#custom-pages-account--custom-page-details) - Details about a specific Custom page details
- [ ] [Update Custom Page](https://api.cloudflare.com/#custom-pages-account--update-custom-page) - Update custom page

[Access Organizations](https://api.cloudflare.com/#access-organizations-properties) - Access Organizations control the look and feel of your login page and the authentication domain it is located at.
- [ ] [Access Organization Details](https://api.cloudflare.com/#access-organizations-access-organization-details) - Fetch your Access Organization details
- [ ] [Create Access Organization](https://api.cloudflare.com/#access-organizations-create-access-organization) - Create a new Access Organization
- [ ] [Update Access Organization](https://api.cloudflare.com/#access-organizations-update-access-organization) - Update a configured Access Organization
- [ ] [Revoke all Access Tokens for a user](https://api.cloudflare.com/#access-organizations-revoke-all-access-tokens-for-a-user) - Revoke any outstanding tokens issued for a specific user

[Access Identity Providers](https://api.cloudflare.com/#access-identity-providers-properties) - Access identity providers are the services your user’s will login against to authenticate with your site
- [ ] [List Access Identity Providers](https://api.cloudflare.com/#access-identity-providers-list-access-identity-providers) - List your Access Identity Providers
- [ ] [Access Identity Providers Details](https://api.cloudflare.com/#access-identity-providers-access-identity-providers-details) - Fetch your Access Identity Providers details
- [ ] [Create Access Identity Provider](https://api.cloudflare.com/#access-identity-providers-create-access-identity-provider) - Create a new Access Identity Provider
- [ ] [Update Access Identity Provider](https://api.cloudflare.com/#access-identity-providers-update-access-identity-provider) - Update a configured Access Identity Provider
- [ ] [Delete Access Identity Provider](https://api.cloudflare.com/#access-identity-providers-delete-access-identity-provider) - Delete an Access Identity Provider

[Access Groups](https://api.cloudflare.com/#access-groups-properties) - Access Groups allow you to define a set of users to which an application policy can be applied. You can reuse Access groups to quickly create policies that apply to the same set of users
- [ ] [List Access Groups](https://api.cloudflare.com/#access-groups-list-access-groups) - List Access Groups
- [ ] [Access Group Details](https://api.cloudflare.com/#access-groups-access-group-details) - Fetch a single Access Group
- [ ] [Create Access Group](https://api.cloudflare.com/#access-groups-create-access-group) - Create a new Access Group
- [ ] [Update Access Group](https://api.cloudflare.com/#access-groups-update-access-group) - Update a configured Access Group
- [ ] [Delete Access Group](https://api.cloudflare.com/#access-groups-delete-access-group) - Delete an Access Group

[Access Service Tokens](https://api.cloudflare.com/#access-service-tokens-properties) - Access Service Tokens allow automated requests to reach protected domains
- [ ] [List Access Service Tokens](https://api.cloudflare.com/#access-service-tokens-list-access-service-tokens) - List Access Service Tokens
- [ ] [Create Access Service Token](https://api.cloudflare.com/#access-service-tokens-create-access-service-token) - Create a new Access Service Token. Note: This is the only time you can get the client secret. Please save it somewhere secure.
- [ ] [Update Access Service Token](https://api.cloudflare.com/#access-service-tokens-update-access-service-token) - Update a configured Access Service Token
- [ ] [Delete Access Service Token](https://api.cloudflare.com/#access-service-tokens-delete-access-service-token) - Delete an Access Service Token

[Access Mutual TLS Authentication](https://api.cloudflare.com/#access-mutual-tls-authentication-properties) - Mutual TLS authentication ensures that the traffic is secure and trusted in both directions between a client and server
- [ ] [List Access Certificates](https://api.cloudflare.com/#access-mutual-tls-authentication-list-access-certificates) - List Access Certificates
- [ ] [Access Certificate Details](https://api.cloudflare.com/#access-mutual-tls-authentication-access-certificate-details) - Fetch a single Access Certificate
- [ ] [Create Access Certificate](https://api.cloudflare.com/#access-mutual-tls-authentication-create-access-certificate) - Create a new Access Certificate
- [ ] [Update Access Certificate](https://api.cloudflare.com/#access-mutual-tls-authentication-update-access-certificate) - Update a configured Access Certificate
- [ ] [Delete Access Certificate](https://api.cloudflare.com/#access-mutual-tls-authentication-delete-access-certificate) - Delete an Access Certificate

[Access Applications](https://api.cloudflare.com/#access-applications-properties) - Access Applications define the routes that Access will block.
- [ ] [List Access Applications](https://api.cloudflare.com/#access-applications-list-access-applications) - List Access Applications for a zone
- [ ] [Access Applications Details](https://api.cloudflare.com/#access-applications-access-applications-details) - Fetch a single Access Application
- [ ] [Create Access Application](https://api.cloudflare.com/#access-applications-create-access-application) - Create a new Access Application
- [ ] [Update Access Application](https://api.cloudflare.com/#access-applications-update-access-application) - Update a configured Access Application
- [ ] [Delete Access Application](https://api.cloudflare.com/#access-applications-delete-access-application) - Delete an Access Application
- [ ] [Revoke Access Tokens](https://api.cloudflare.com/#access-applications-revoke-access-tokens) - Revoke any outstanding tokens issued for the Application.

[Access Policy](https://api.cloudflare.com/#access-policy-properties) - Access Policies define the users or groups who can, or cannot, reach the Application Resource.
- [ ] [List Access Policies](https://api.cloudflare.com/#access-policy-list-access-policies) - List Access Policies for an Access Application
- [ ] [Access Policy Details](https://api.cloudflare.com/#access-policy-access-policy-details) - Fetch a single Access Policy
- [ ] [Create Access Policy](https://api.cloudflare.com/#access-policy-create-access-policy) - Create a new Access Policy
- [ ] [Update Access Policy](https://api.cloudflare.com/#access-policy-update-access-policy) - Update a configured Access Policy
- [ ] [Delete Access Policy](https://api.cloudflare.com/#access-policy-delete-access-policy) - Delete an Access Policy

[Health Checks](https://api.cloudflare.com/#health-checks-properties) - Zone-level stand-alone health checks
- [ ] [List Health Checks](https://api.cloudflare.com/#health-checks-list-health-checks) - List configured health checks
- [ ] [Create Health Check](https://api.cloudflare.com/#health-checks-create-health-check) - Create a new health check
- [ ] [Health Check Details](https://api.cloudflare.com/#health-checks-health-check-details) - Fetch a single configured health check
- [ ] [Update Health Check](https://api.cloudflare.com/#health-checks-update-health-check) - Update a configured health check
- [ ] [Delete Health Check](https://api.cloudflare.com/#health-checks-delete-health-check) - Delete a health check

[Registrar Domains](https://api.cloudflare.com/#registrar-domains-properties) - Manage Domains Names
- [ ] [List Domains](https://api.cloudflare.com/#registrar-domains-list-domains) - List domains handled by registrar
- [ ] [Get Domain](https://api.cloudflare.com/#registrar-domains-get-domain) - Show individual domain
- [ ] [Update Domain](https://api.cloudflare.com/#registrar-domains-update-domain) - Update individual domain