# Nuxt UI Pro Bypass V2

> [!WARNING]  
> TThis script is intended only for demonstrating the problem and should not be used in production.

> [!CAUTION]
> Never use this in production — it may be illegal. :)

### Instructions
1. Install Docker on your system.
2. Build the bypass image:

    ```
    docker build https://github.com/qweme32/nuxt-ui-pro-license-bypass.git#main -t nuxt-pro-ui-bypass
    ```
3. In your Nuxt project, create a `.env` file with the following content:

    ```
    # Production license for @nuxt/ui-pro, get one at https://ui.nuxt.com/pro/purchase
    NUXT_UI_PRO_LICENSE=XD_BYPASSED
    ```
4. Run the bypass:

    ```
    docker run --rm -v "$(pwd)/node_modules:/node_modules" nuxt-pro-ui-bypass
    ```

### Change log
- `Aug 21, 2024`

    First bypass written as a small Python script.
- `June 10, 2025`

    Nuxt rewrote some build logic and added chunks. Second bypass rewritten in Rust and dockerized for easy use.
    
---

> Enjoy 😉
