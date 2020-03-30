import App from './App.svelte';

const app = new App({
    target: document.getElementById("container"),
    props: {
        tab: "upload"
    }
});

export default app;
