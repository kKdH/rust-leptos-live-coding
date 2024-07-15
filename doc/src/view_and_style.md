# View with Style!

1. Create a variable `name` and interpolate it into the heading
2. Change the color of the displayed name with a custom `name_style`.
3. Set a css class.
   ```rust
   #[component]
   pub fn App() -> impl IntoView {
       let name = "Elmar"; // (1)
       let style = "color: blue;"; // (2)
       view! {
           <h1>"Hello "
               <span
                   class="name-label" // (3)
                   style=style // (2)
                >
                   {name}  // (1)
               </span>
          </h1>
       }
   }
   ```

4. Create the css class:
   `styles.scss`
   ```scss
   .name-label {
     font-family: "monospace"
   }
   ```

1. Extend the `index.html` to include the style.
    ```html
    <!DOCTYPE html>
    <html>
      <head>
        <link data-trunk rel="scss" href="public/styles.scss" />
        <link data-trunk rel="icon" href="public/favicon.png" />
      </head>
      <body></body>
    </html>
    ```
