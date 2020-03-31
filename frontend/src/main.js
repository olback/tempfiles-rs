import App from './components/App.svelte';

const app = new App({
    target: document.getElementById("container"),
    props: {
        tab: "upload"
    }
});

export default app;
