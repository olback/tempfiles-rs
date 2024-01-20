<script>
    import { createEventDispatcher } from "svelte";
    export let tab;

    const dispatch = createEventDispatcher();

    function setTab(tab) {
        dispatch("setTab", tab);
    }
</script>

<header>
    <div class="header-container">
        <h1>
            <a href="/"> <i class="fas fa-user-ninja"></i> Tempfiles</a>
        </h1>
        <div class="buttons">
            <button
                on:click={() => setTab("upload")}
                class={tab === "upload" ? "selected" : ""}
                ><i class="fas fa-file-upload"></i> Upload</button
            >
            <button
                on:click={() => setTab("download")}
                class={tab === "download" ? "selected" : ""}
                ><i class="fas fa-file-download"></i> Download</button
            >
            <button
                on:click={() => setTab("delete")}
                class={tab === "delete" ? "selected" : ""}
                ><i class="fas fa-eraser"></i> Delete</button
            >
            <button
                on:click={() => setTab("sharex")}
                class={tab === "sharex" ? "selected" : ""}
                ><i class="fas fa-share-square"></i> ShareX</button
            >
            <button
                on:click={() => setTab("api")}
                class={tab === "api" ? "selected" : ""}
                ><i class="fas fa-code"></i> API</button
            >
        </div>
    </div>
</header>

<style type="text/scss">
    @import "src/scss/variables";

    header {
        grid-column-start: 1;
        grid-column-end: 4;
        grid-row-start: 1;
        grid-row-end: 1;
        grid-template-columns: 25vw auto 25vw;
        display: grid;
        background-color: $accent;
        color: $background;

        div.header-container {
            grid-column-start: 2;
            grid-column-end: 3;
            display: grid;
            grid-template-columns: auto auto;
            align-self: stretch;
            align-self: center;
            padding: 0;
            margin: 0;

            * {
                color: $background;
                margin: 0;
                align-self: center;
                text-decoration: none;
            }

            div.buttons {
                justify-self: right;

                button {
                    border: none;
                    background-color: transparent;
                    outline: none;
                    padding: 1em;
                    font-size: 12pt;
                    font-weight: bold;

                    &:hover,
                    &.selected,
                    &:hover > i,
                    &.selected > i {
                        background-color: $background;
                        color: $accent;
                        cursor: pointer;
                    }
                }
            }
        }
    }
</style>
