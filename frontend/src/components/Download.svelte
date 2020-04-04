<script>

    let id = "";
    let password = "";

    function download() {

        window.location = `${window.origin}/d/${id}/${password}`;

    }

</script>

<h1>Download</h1>

<div class="delete">

    <label for="id">ID</label>
    <input bind:value={id} id="id" type="text">

    <label for="password">Password</label>
    <input bind:value={password} id="password" type="text">

    <button on:click={download}>Download</button>

</div>

<style type="text/scss">

    @import 'src/scss/variables';

    h1 {
        color: $accent;
    }

    div.delete {

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

            width: 100%;
            background-color: $accent;
            color: $background;
            border: none;
            padding: 1em;
            font-size: 12pt;
            outline: none;
            cursor: pointer;
            transition: background-color $transition_time_button ease-in-out;
            grid-column-start: 1;
            grid-column-end: 3;

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

    }

</style>
