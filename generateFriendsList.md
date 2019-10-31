At their heart, social networks are a special form of graph data structure. Humans constitute nodes on the social graph, and relationships between humans are edges connecting those nodes.

Most social network algorithms depend on these edges. When you create a post on Facebook, your post is fed into the Newsfeed algorithm of your friends. The more that your friends engage with your content, the higher the post will appear in the newsfeed of each of your friends. Also depending on your privacy settings, your friends can share your comments, which cascades your posts out to their friends (your friend-of-friends). When this occurs recursively, this is "viral" social media.

With all this in mind, your most important and valuable data on a social network is thus likley not your profile information, but your friend connections which define your role in the social graph.

Facebook's Data Export Tool seems to be quite useful at exporting profile information and all of the Facebook metadata that Facebook tags you with for advertising and tracking purposes. By downloading your personal data, you can get a good grasp of the data that Facebook has on you, and that can help you make informed decisions about your privacy settings and the degree to which your are comfortable being tracked while using Facebook. 

Looking through the zip file containing JSON of much of my profile, one thing seems to be conspicuously absent: Your friend list.

While this data likely isn't important for the use case of practicing good privacy hygiene on Facebook, it is critical for bootstrap relationships on SOLID or any other alternative social networks (which is likely why Facebook excludes this data).

In SOLID, humans are represented by cannonical URLs that resolve to their profile data and connections are encoded in SOLID profile using the foaf:knows relationship. The neat thing about this is that it's super flexible. We can express our connections as abstract foaf Persons within our profile that include their name and a link to their facebook profiles.

Theoretically, we could also have a reverse lookup web service that maps cannonical facebook profile URLs to cannoninical SOLID URLs, so that when a user opens a SOLID pod, they can find the SOLID URL of their Facebook friends.

To get around this, we can scrape this data off our Facebook profiles using browser DevTools and a bit of JavaScript.

1. Go to https://www.facebook.com/bushidocodes/friends. The page lazy-loads friends for performance reasons, so scroll to the bottom of the friends section until all friends are listed. You should see a page break showing "More About You" when you are at the bottom of the friends section
2. Open your browser DevTools to the console (Ctrl+Shift+j on Chromium on Windows)
3. Review the following JavaScript code to trust that I'm not doing anything nefarious. The script essentially extracts your list of friends and the URL of their Facebook profiles. 
```js
JSON.stringify([...document.querySelectorAll("[data-testid = 'friend_list_item']")].map(el => [...el.querySelectorAll("*")].filter(child => child.className === "fsl fwb fcb")[0].firstChild).map(el => ({name: el.textContent, target: `${el.getAttribute("href")}`.split("?")[0]})))
```
4. After executing, you should see the first bit of the resulting JSON object. At the bottom (in Chromium), you have a Copy button. Click to copy this data.
![Image showing the copy button in devtools](./docs/copy-json.jpg)
5. Use a code editor to paste this data into a `friends.json` file
