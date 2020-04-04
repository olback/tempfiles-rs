<script>

    import config from '../config.js';

    let id = "";
    let delete_password = "";
    let success = null;
    let error = null;

    function doDelete() {

        const url = config.apiUrl.join(`delete/${id}/${delete_password}`);

        console.log(url.toString());

        fetch(url.toString(), {
            method: 'DELETE'
        })
        .then(res => {
            if (res.status === 200) {
                success = "File deleted";
                id = "";
                delete_password = "";
            } else {
                error = "File not found";
            }
        })
        .catch(err => error = err);

    }


</script>

<h1>Delete</h1>

<div class="delete">

    <label for="id">ID</label>
    <input bind:value={id} id="id" name="id" type="text">

    <label for="delete_password">Deletion password</label>
    <input bind:value={delete_password} id="delete_password" name="delete_password" type="text">

    <button on:click={doDelete}>Delete</button>

    {#if success}
        <p class="success">Success: {success}</p>
    {/if}

    {#if error}
        <p class="error">Error: {error}</p>
    {/if}

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

        p {

            text-align: center;
            padding: 1em;
            color: $background;
            grid-column-start: 1;
            grid-column-end: 3;

            &.success {
                background-color: #5f5;
            }

            &.error {
                background-color: #f33;
            }

        }

    }

</style>
