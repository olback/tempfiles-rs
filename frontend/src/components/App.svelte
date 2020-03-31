<script>

    export let tab;

    // Site components
    import Header from './Header.svelte';
    import Footer from './Footer.svelte';

    // Tabs
    import Upload from './Upload.svelte';
    import Download from './Download.svelte';
    import Delete from './Delete.svelte';
    import ShareX from './ShareX.svelte';
    import Api from './Api.svelte';

    // On load, show the requested tab
    const l = location.pathname.substring(1);
    switch (l) {

        case 'upload': {
            setTab(l);
            break;
        }

        case 'download': {
            setTab(l);
            break;
        }

        case 'download': {
            setTab(l);
            break;
        }

        case 'delete': {
            setTab(l);
            break;
        }

        case 'sharex': {
            setTab(l);
            break;
        }

        case 'api': {
            setTab(l);
            break;
        }

        default: {
            history.pushState({}, '/', '/');
            tab = 'upload';
        }

    }

    function setTab(path) {
        history.pushState({}, path, path);
        tab = path;
    }

    function setTabEvent(event) {
        history.pushState({}, event.detail, event.detail);
        tab = event.detail;
    }

</script>

<Header on:setTab={tab = setTabEvent} tab={tab} />

<main>

    <div class="content">

        {#if tab === "upload"}
            <Upload />
        {:else if tab === "download"}
            <Download />
        {:else if tab === "delete"}
            <Delete />
        {:else if tab === "sharex"}
            <ShareX />
        {:else if tab === "api"}
            <Api />
        {:else}
            <p>Unreachable</p>
        {/if}

    </div>

</main>

<Footer />

<style type="text/scss">

    @import 'src/scss/variables';

    main {

        grid-column-start: 2;
        grid-column-end: 3;
        grid-row-start: 2;
        grid-row-end: 3;
        justify-items: center;
        justify-content: stretch;
        display: grid;

        div.content {

            align-self: center;
            // justify-items: center;
            justify-content: stretch;
            // display: grid;
            width: 600px;

        }

    }

</style>
