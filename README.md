# Fake Discord embed generator

A small webserver written in Rust that returns a different page depending on the user-agent.

If e.g. Discord preview generator bot visits the site, it gets served a webpage with certain meta tags to generate a preview.

A real user visiting the webpage will instead be redirected to [this YouTube video](https://www.youtube.com/watch?v=dQw4w9WgXcQ).
