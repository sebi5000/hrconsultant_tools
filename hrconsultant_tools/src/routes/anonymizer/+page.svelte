<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { appWindow } from '@tauri-apps/api/window';

    let anonymizationIsWorking = false;
    $:anonymizationIsWorking;

    const uploadProgressEvent = appWindow.listen<string>('chooseFileDoneEvent', (event) =>{
        anonymizationIsWorking = false;
        console.log("Test!");
    });

    async function chooseFile(){
        anonymizationIsWorking = true;
        await invoke("choose_file");
    }

</script>

<div>
    <h3>How it works!?</h3>
    <ol>
        <li>Auswählen einer Datei für die Anonymisierung (zurzeit mögliche Formate: xlsx)</li>
        <li>Anonymisierung erfolgt sofort nach Auswahl</li>
        <li>Speichern-Dialog erlaubt das Abspeichern einer Kopie</li>
    </ol>
</div>
<div>
    <button disabled={anonymizationIsWorking} class="primary" on:click="{chooseFile}">Datei auswählen</button>
    {#if anonymizationIsWorking}
        <div class="spinner primary"></div>
        <p>We are anonymizing the file</p>
    {/if}
</div>