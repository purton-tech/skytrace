+++
title = "Landing Page"
weight = 40
template = "docs-page.html"
+++

## Creating your own landing page

Copy the existing file from `theme/tech-startup-theme/templaes/index.html` into your `templates` folder.

The landing page is created in HTML. 

## Adding your own content

For the content you want to add you can add pre-configured sections. For example.

## Hero Section


```html
<section class="m_cta">
  <div class="content">
    <h1>The Tech Startup theme gives you the basic static site structure for your Saas or application</h1>
    <div class="text">
      <p>Save time and focus on your product, let the Tech Startup theme handle marketing.
      </p>
    </div>
    <div class="buttons"><a class="a_button a_button--primary" href="{{ sign_up_url }}">Sign Up</a></div>
    <div class="image"><img alt="Cream" src="startup.svg"></img></div>
  </div>
</section>
```

## Screenshot Section

```html
<section class="m_screenshot">
  <div>
    <h2>The Tech Startup theme is used by our signature product Cloak</h2>
    <h4>
      The theme can be configured to match your product colours and message
    </h4>
    <img alt="Cream" src="cloak-screenshot.png" />
  </div>
</section>
```