+++
title = "Styling"
weight = 30
template = "docs-page.html"
+++

## Overriding the css

Create a folder called `sass` if one doesn't already exists and add the following in a file called `index.scss`.

```sass
@import '../themes/tech-startup-theme/sass/index';


// Colours
:root {
    --foreground: #000;
    --background: #f54444;
    --accents-1: #fafafa;
    --accents-2: #EBEBEB;
    --accents-3: #999999;
    --accents-4: #888888;
    --accents-5: #666666;
    --accents-6: #444444;
    --accents-7: #333333;
    --accents-8: #111111;
    --primary: #000;
    --success: #031b4d;
    --error: #ee0000;
    --warning: #f5a623;
    --dropdown-box-shadow: 0 4px 4px 0 rgba(0, 0, 0, 0.02);
    --dropdown-triangle-stroke: #fff;
    --scroller-start: var(--background);
    --scroller-end: rgba(255, 255, 255, 0);
    --shadow-small: 0 5px 10px rgba(0, 0, 0, 0.12);
    --shadow-medium: 0 8px 30px rgba(0, 0, 0, 0.12);
    --shadow-large: 0 30px 60px rgba(0, 0, 0, 0.12);
}
```

You can now override colors and other style attributes.