<script>

    import config from '../config.js';
    import { ParamsBuilder } from '../params-builder.js';

    let label;
    let input;
    let error = null;

    let filename = null;

    function onInput(evt) {
        filename = evt.target.value.replace('C:\\fakepath\\', '');
        label.innerText = filename;
        label.classList.remove('none');
    }

    function uploadFile() {

        if (filename === null) {
            error = "Please select a file before uploading";
            return;
        }

        let params = new ParamsBuilder();
        params.append('filename', filename);
        // params.append('maxviews', 28);

        const url = config.apiUrl.join('upload');
        url.search = params.toString();

        fetch(url.toString(), {
            method: 'POST',
            body: input.files[0]
        })
        .then(res => res.json())
        .then(json => {
            console.log(json);
        })
        .catch(err => error = err);

    }

</script>

<h1>Upload a file</h1>

<div class="file-input">
    <input bind:this={input} on:input={onInput} type="file" id="file-input">
    <label bind:this={label} for="file-input" class="none">Choose file...</label>
</div>

<button on:click={uploadFile}>Upload</button>

{#if error}
    <p class="error">{error}</p>
{/if}

<style type="text/scss">

    @import 'src/scss/variables';

    h1 {
        color: $accent;
    }

    div.file-input {

        margin: 2em 0;

        input[type=file] {
            display: none;
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
    }

    button:hover {
        background-color: $accent2;
    }

    p.error {
        text-align: center;
        background-color: #f33;
        padding: 1em;
        color: $background;
    }

</style>
