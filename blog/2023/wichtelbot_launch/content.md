# Launching Wichtelbot.com

The joy of Christmas is undeniable, but the task of finding gifts for an ever-growing family can be daunting. That's why I've implemented a solution that has not only been successful in my own family for three consecutive years, but is also adaptable for any family or group.

The answer lies in the delightful tradition of Secret Santa, a game that adds excitement to the holiday season while easing the stress of gift-giving. In Germany, it is called "Wichteln".

The only hiccup: Traditionally, you’re required to convene in person with your group. The complexity escalates if you wish to leave a notice or have couples who prefer to exclude each other. The apps currently available are typically inundated with ads/tracking and superfluous features, leading to a less than pleasant user experience.

That’s why I took matters into my own hands and created a streamlined app, crafted in GO with an emphasis on performance and user-friendliness.

[Try it live on wichtelbot.com](https://www.wichtelbot.com)

![test](/media/wichtelbot-launch/wichtelbot.jpeg)

[Want to dive into the code or self-host your own?](https://github.com/Lommix/WichtelBotTheSequel)

Written in GO, utilizing solely the standard libraries, with the singular exception of a SQLite driver. Htmx for a traditional approach to server-side state management, complemented by some Tailwind and vanilla JS to ensure it’s visually appealing.

## Features

- Free & Open source. No Ads, no tracking.
- Notice: Each player can leave a notice. Mainly used for allergies and picky family members.
- Blacklist: Each player can blacklist another. This feature has to be enabled by the moderator.
- Multi language support. Renders German if your browser is German, else English.
- A very cute animated gopher.
