<script>

    import config from '../config.js';
    import { ParamsBuilder } from '../params-builder.js';

    let label;
    let input;
    let maxViews = 26;
    let maxViewsLabel;
    let error = null;
    let filename = null;
    let upload_success = null;
    let loading = false;

    function onInput(evt) {

        filename = evt.target.files[0].name;
        label.innerText = filename;
        label.classList.remove('none');

    }

    function getContentType(file) {

        if (file.type.includes('text')) {
            return `${file.type};charset=UTF-8`
        } else {
            return file.type
        }


    }

    function uploadFile(evt) {

        error = null;

        if (filename === null) {
            error = "Please select a file before uploading";
            return;
        }

        evt.target.disabled = true;
        loading = true;

        let params = new ParamsBuilder();
        params.append('filename', filename);
        if (maxViews < 26) {
            params.append('maxviews', maxViews);
        }

        const url = config.apiUrl.join('upload');
        url.search = params.toString();

        fetch(url.toString(), {
            method: 'POST',
            body: input.files[0],
            headers: {
                'Content-Type': getContentType(input.files[0])
            }
        })
        .then(res => res.json())
        .then(json => {
            if (json.status === 201) {
                upload_success = json;
                loading = false;
            } else {
                evt.target.disabled = false;
                loading = false;
                error = json.message;
            }
        })
        .catch(err => error = err);

    }

    function setClipboard(el) {

        el.target.select();
        el.target.setSelectionRange(0, 1000);

        document.execCommand('copy');

    }

</script>

{#if upload_success}

    <h1>Success!</h1>

    <div class="success">

        <label for="success-url">Link</label>
        <input on:click={setClipboard} id="success-url" type="text" readonly value="{upload_success.download_url}">

        <label for="success-id">ID</label>
        <input on:click={setClipboard} id="success-id" type="text" readonly value={upload_success.id}>

        <label for="success-delete">Deletion password</label>
        <input on:click={setClipboard} id="success-delete" type="text" readonly value={upload_success.delete_password}>

        <button on:click={() => {upload_success = null; error = null; filename = null; maxViews = 26}}>
            Upload another
        </button>

    </div>

{:else}

    <h1>Upload a file</h1>

    <h2>{document.querySelector('meta[property="og:description"]').content}</h2>

    <div class="upload">

        <div class="file-input">
            <input bind:this={input} on:input={onInput} type="file" id="file-input">
            <label bind:this={label} for="file-input" class="none">Choose file...</label>
        </div>

        <div class="max-views">
            <span>Max views</span>
            <input bind:value={maxViews} on:input={(e) => {maxViewsLabel.innerHTML = e.target.value == 26 ? 'Infinite' : e.target.value}} type="range" min="1" max="26" step="1">
            <span bind:this={maxViewsLabel}>Infinite</span>
        </div>

        <button on:click={uploadFile}>
            {#if loading}
                <span class="fas fa-sync fa-spin"></span>
            {:else}
                Upload
            {/if}
        </button>

        {#if error}
            <p class="error">Error: {error}</p>
        {/if}

    </div>

{/if}

<style type="text/scss">

    @import 'src/scss/variables';

    h1, h2 {
        color: $accent;
    }

    h2 {
        font-size: 16pt;
        font-weight: 400;
    }

    div.success {

        display: grid;
        grid-template-columns: auto auto;
        row-gap: 1em;
        align-content: center;
        align-items: center;

        label {
            color: $accent;
            font-weight: bold;
        }

        input {

            padding: 1em;
            background-color: #ddd;
            width: auto;
            display: block;
            border: none;
            color: $background;
            outline: none;
            font-size: 12pt;

        }

        button {
            width: max-content;
        }

    }

    div.upload {

        display: grid;
        row-gap: 1.5em;

        div.max-views {

            color: $accent;
            font-size: 12pt;
            display: grid;
            grid-template-columns: 25% auto 25%;
            justify-content: stretch;
            justify-items: center;
            font-weight: bold;

            input[type=range] {

                justify-self: stretch;
                -webkit-appearance: none;
                outline: none;
                background-color: transparent;

                &::-webkit-slider-runnable-track {
                    // margin-bottom: 10px;
                    width: 100%;
                    height: 5px;
                    cursor: pointer;
                    box-shadow: none;
                    background: $background2;
                    border-radius: 25px;
                    border: none;
                }

                &::-moz-range-track {
                    width: 100%;
                    height: 5px;
                    cursor: pointer;
                    box-shadow: none;
                    background: $background2;
                    border-radius: 25px;
                    border: none;
                }

                &::-webkit-slider-thumb {
                    box-shadow: none;
                    border: none;
                    height: 20px;
                    width: 20px;
                    border-radius: 50%;
                    background: $accent2;
                    cursor: pointer;
                    -webkit-appearance: none;
                    margin-top: -7.6px;
                }

                &::-moz-range-thumb {
                    box-shadow: none;
                    border: none;
                    height: 20px;
                    width: 20px;
                    border: none;
                    border-radius: 50%;
                    background: $accent2;
                    cursor: pointer;
                    -webkit-appearance: none;
                    margin-top: -3.6px;
                }

            }

        }

    }

    div.file-input {

        // margin: 2em 0;

        input[type=file] {
            display: none;
        }

        input[type=range] {
            display: inline-block;
            margin: auto;
            margin-top: 2em;
            width: 50%;
        }

        label {

            padding: .7em 0;
            background-color: #ddd;
            cursor: pointer;
            width: auto;
            display: block;

            &::before {
                font-style: normal;
                color: $background;
                content: "Browse";
                padding: .7em;
                margin-right: 1em;
                background-color: $accent;
                transition: background-color $transition_time_button ease-in-out;
            }

            &:hover::before {
                background-color: $accent2;
            }

            &.none {
                font-style: italic;
                color: #555;
            }

        }

    }

    button {
        width: 100%;
        background-color: $accent;
        color: $background;
        border: none;
        padding: 1em;
        font-size: 12pt;
        outline: none;
        cursor: pointer;
        transition: background-color $transition_time_button ease-in-out;

        &:hover:not(:disabled) {
            background-color: $accent2;
        }

        &:disabled {
            background-color: $accent2;
            cursor: initial;
        }

    }

    p.error {
        text-align: center;
        background-color: #f33;
        padding: 1em;
        color: $background;
    }

</style>
