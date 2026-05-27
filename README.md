# Hatchling

_Break out of the Facebook Shell by converting your social Data into Turtle Triples_

![Dancing Hatchling](https://media.giphy.com/media/M7Txf8Imy1pGo/giphy.gif)


[Watch my YouTube video on this project!](https://youtu.be/m9_CwF9SUuY)

I'm a fan of Facebook. They create social networks that my grandparents are able to use, and they contribute heavily to the open-source ecosystem. That said, I'm a bigger fan of user empowerment and choice.

Out of the alternatives today, I believe that Tim Berners-Lee's vision for [SOLID (a decentralized social network built around linked-data principles)](https://solid.mit.edu/) offers the most pragmatic vision for data ownership. It offers the possibility that existing social networks could work with the standards bodies and adopt linked data and the SOLID data ownership model. 

However, for this to be a possibility, tech early adopters need to engage with the linked data ecosystem and learn the fundamental of the semantic web. 

This project aims to encourage SOLID adoption by simplifying the process of expressing a profile using linked data and Turtle. It provides a binary that converting provided by Facebook's [Download Your Information](https://www.facebook.com/dyi/?x=AdkiqAMlydfH5oKw) into semantic web triples used by SOLID. 

Note: If you want to learn more about the theory beTo learn more about this, I suggest reading [Linked Data Fundamentals](https://solid.inrupt.com/docs/intro-to-linked-data) on the SOLID website.

## Step 1: Download Your Data from Facebook

1. Navigate to [Meta Accounts Center → Your information and permissions → Download your information](https://accountscenter.facebook.com/info_and_permissions/dyi/).
2. Click **Export your information** → **Create export**.
3. Under **Specific types of information**, check only **Profile information** (to keep the export small and fast).
4. Set Format → **JSON**, Date range → **All time**, Media quality → **Low**.
5. Click **Submit request** and wait for the email notification (usually a few minutes for profile-only exports).
6. Download the zip file and extract it.
7. The profile file is at: `<extracted-folder>/personal_information/profile_information/profile_information.json`

## Step 2: [Optional] Export your connections with profile URLs

Facebook's "Download Your Information" export includes a `friends_v2` list, but it only contains friend names — no profile URLs. Realistically, services require a globally unique identifier to identify someone on the web, which for Facebook is the public URL of their profile.

`"John Smith" is not useful, but https://www.facebook.com/johnjohn.smith.12345 is.`

To get the URLs, we scrape your friends page using browser DevTools and a bit of JavaScript.

1. Go to `https://www.facebook.com/<your-username>/friends`. The page lazy-loads friends, so **scroll to the very bottom** of the friends list until all friends are loaded.
2. Open browser DevTools to the Console tab (Ctrl+Shift+J on Chromium/Windows, Cmd+Option+J on Mac).
3. Review the following JavaScript — it finds all profile links on the page, excludes your own, and deduplicates:
```js
JSON.stringify((function(){const my=location.href.replace('/friends','').split('?')[0];const seen=new Set();return[...document.querySelectorAll('a')].filter(a=>{const h=a.href||'';return(h.match(/facebook\.com\/[a-zA-Z0-9._]+$/)||h.match(/facebook\.com\/profile\.php\?id=\d+/))&&a.innerText.trim().length>1&&!h.includes(my);}).map(a=>({name:a.innerText.trim(),target:a.href.match(/profile\.php/)?a.href:a.href.split('?')[0]})).filter(f=>!seen.has(f.target)&&seen.add(f.target));}()))
```
4. After executing, you'll see the JSON result. In Chromium, right-click the output → **Copy string contents** (or click the Copy button at the bottom of the console output).
5. Paste the copied data into a `friends.json` file.

## Step 3: Download and Run Binary

1. [Download the binary for the appropriate platform and architecture](https://github.com/bushidocodes/hatchling/releases) (at this time, only x64 Linux and Windows are supported).
2. Start a command prompt and run one of:

**Profile only** (no friends):
```
hatchling.exe path/to/profile_information.json out.ttl
```

**With friends from the DYI export** (names only, no profile URLs):
```
hatchling.exe path/to/profile_information.json out.ttl --friends path/to/your_friends.json
```
The DYI friends file is at: `<extracted-folder>/connections/friends/your_friends.json`

**With friends scraped via Step 2** (includes profile URLs):
```
hatchling.exe path/to/profile_information.json out.ttl --friends friends.json
```

## Step 4: Validate and Edit Output file

Now that you've generated a Turtle file, you should verbally inspect the output and manually remove privileged data.

When done, you can validate your file for correct syntax [with the W3C validator](https://www.w3.org/2015/03/ShExValidata/)
