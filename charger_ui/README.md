# charger_ui

This is a simple UI to display the input and output of the simulation software contained in
[`charging_model`](./../charging_model/).
Make sure to run `npm install` before trying to run or build the UI.

## Running the UI

```bash
# run the dev server
npm run dev
```

## Building

```bash
npm run build
```

## Deploying

This sample UI is designed for `SvelteKit`'s static adapter, which means it produces fully pre-rendered artifacts.
The produced static files can be found in the [`./build`](./build) directory once the build command is run.

You can preview the production build with `npm run preview`.


# Todo

- Implement all inputs on the sim page as part of one form.
- On click, post the form contents to the server.
- Keep implementing the other views (simulation start button, output overview, get quote)
