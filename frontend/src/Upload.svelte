<script></script>

<h1>Upload file</h1>

<div class="file-input">
    <input type="file" name="" id="file-input">
    <label for="file-input" class="none">Choose file...</label>
</div>

<button>Upload</button>

<style type="text/scss">

    @import 'src/scss/colors';

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
    }

</style>
