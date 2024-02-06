# Evilreader Beta

I find great satisfaction in engaging with artificial intelligence, particularly within contexts where its application is logical and impactful.
One area where this holds true is in the realm of text-to-speech technology.

With [Evilreader](https://evilreader.de) I have created a web application that allows users to integrate a widget
into any website and have an AI read out the content of the page to them.

Currently, the application is in beta, and I am looking for feedback from users.
It is intended to be used by people with visual impairments, but it can also be used by anyone who wants to listen to the content of a website.

In its current iteration, the application has to be installed by the website owner, but
this is only the beginning. I already have a prototype for a Browser extension that allows users to
install the application themselves and use it on any website.

I can't control how other sites show their information, which might make it hard to read things in the right order. I'm trying to fix this, but for now, I only have a basic version of a solution. I'll keep working on it and let you know how it goes.

For now, you can check out the [Evilreader website](https://evilreader.de/docs) and try it out for yourself.
Very soon I will implement this into my blog as well.

## How it works

There are two parts to the application: The widget and the backend. The widget makes use
of a standard HTML5/JS Webcomponent and can be installed on any website.
I choose Webcomponents, because there is no bloat they are easy to maintain and use.

Like any other HTML5 element, they can be styled and interacted with using CSS and JS.

Here is the simple example from the [Evilreader website](https://evilreader.de/docs):

```html
<script src="https://cdn.evilreader.com/narator.min.js"></script>

<narator-element
    plays="h1, h2, p, .read-this, #read-that"
    api-url="https://api.evilreader.de"
    focus-class="currently-reading"
    primary-color="#ff0000"
    secondary-color="#0000ff"
    hide-player="false"
></narator-element>
```

![Evilreader Widget](/media/evilreader-beta/widget.jpeg)

The second part is the backend. A highly scalable API written in Rust that handles the generation, caching and streaming
of audio files.

Later on, I want to make a service where if you give it some recordings of your voice, you can make your own models and use them with the app.

Currently, I am looking for some companies that would be interested in working together on
getting the application ready for production. If you are interested, please contact me at [evilreader.de](https://evilreader.de/contact)
