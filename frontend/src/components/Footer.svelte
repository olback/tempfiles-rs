<script>
    let version = document.querySelector('meta[name=version]').content;
</script>

<footer>
    <p>
        Author: <a href="https://github.com/olback">olback</a>
        <span class="spacer"></span>
        Version: <a href="https://github.com/olback/tempfiles-rs" title={version}>{version.substring(0, 7)}</a>
    </p>
</footer>

<style type="text/scss">

    @import 'src/scss/variables';

    footer {

        grid-column-start: 1;
        grid-column-end: 4;
        grid-row-start: 3;
        grid-row-end: 3;
        align-self: stretch;
        justify-items: center;
        justify-content: center;
        display: grid;
        padding: 1rem;
        border-bottom: solid 5px $accent;

        p, a {
            color: $accent;
            align-self: center;
            padding: 0;
            margin: 0;
            transition: color $transition_time_link ease-in-out;
        }

        a:hover {
            color: $accent2;
        }

        span.spacer {
            font-size: 12pt;
            display: inline-block;
            width: 1rem;
        }

    }

</style>
