<script lang="ts">
    import 'mini.css/dist/mini-default.css'
    import { appWindow } from '@tauri-apps/api/window';

    const handleMenuBar = appWindow.listen('MenuBar', ({event, payload}) => {
        console.log(payload);
    });

</script>

<div style="min-height:100vh; display:flex; flex-direction:column; justify-content:space-between;">
    <header>
        <a href="/" class="button">Start</a>
        <a href="/anonymizer" class="button">Anonymizer</a>
    </header>
    <slot />
    <footer class="sticky" style="text-align:center;">
        <small>HR Consultant Tools v.0.2 | Copyright Sebastian Eßling</small>
    </footer>
</div>