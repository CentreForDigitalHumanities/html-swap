## 🚴 Usage


### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --target web
```


### Include in your base HTML

```html
<script src='pkg/html_swap.js'></script>
<script>
    const { swap_ctl } = wasm_bindgen;
    async function run() {
    await wasm_bindgen();
    }
    run();
</script>
```

Then in your HTML mark elements that serve as a swap controller by setting the `data-swap-ctl` property. 
The supported tags are `a` tags which will issue a get request and `input` elements with a type of submit which will perform the relevant form action.

```html
<form method="get" action="/search">
    <input type="text" name="q" />
    <input type="submit" onclick="swap_ctl(this); return false;" value="Search" />
</form>
```

```html
<a href="/example" onclick="swap_ctl(this); return false;">Link</a>
```

This will prevent default behavior of showing the response and instead will update the page with the response.
A response is comprised of a series of updates that apply to slots on the page. 

For instance, consider the following `index.html`

```html
<a href="example.html" onclick="swap_ctl(this); return false;">Link</a>
<div id="slot-1">Default content</div>
```

And the following `example.html`

```html
<div id="slot-1">Updated Content</div>
```

By clicking the anchor tag you will replace the inner html of the initial slot with the content in the new slot. 
You can perform multiple updates by providing multiple elements in the same response:

`index.html`

```html
<a href="example.html" onclick="swap_ctl(this); return false;">Link</a>
<div id="slot-1">Default content</div>
<div id="slot-2">Default content</div>
```

`example.html`

```html
<div id="slot-1">Update 1</div>
<div id="slot-2">Update 2</div>
```