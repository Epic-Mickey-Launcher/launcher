<script lang="ts">
    import {mount, onMount, unmount} from "svelte";
    import {SetHeaderVisible} from "./library/config";
    import ModInstall from "./components/ModInstall.svelte";
    import {DownloadDolphin} from "./library/dolphin";

    let mickeyLogo: HTMLDivElement = $state()
    let initialSetup: HTMLDivElement = $state()
    let background: HTMLDivElement = $state()
    let installDolphin = $state(true)
    onMount(async () => {
        SetHeaderVisible(false);

        background.classList.toggle("DisableBackground")
        mickeyLogo.classList.toggle("MickeyLogoFinished")
        mickeyLogo.classList.toggle("MickeyLogoFinishedDisappear")
        initialSetup.classList.toggle("InitialSetupFinished")

        setTimeout(() => {
            mickeyLogo.classList.toggle("MickeyLogoFinished")
        }, 15)

        setTimeout(() => {
            mickeyLogo.classList.toggle("MickeyLogoFinishedDisappear")
            initialSetup.style.display = "block";

            setTimeout(() => {
                initialSetup.classList.toggle("InitialSetupFinished")
            }, 500)

        }, 1500)
    })

    async function Proceed() {
        initialSetup.classList.toggle("InitialSetupFinished")

        if (installDolphin) {
            let modInstallElement = mount(ModInstall, {
                target: document.body,
            });
            modInstallElement.modName = "Dolphin";
            modInstallElement.modIcon = "img/dolphin.png";
            modInstallElement.showDownloadProgression = true;
            await DownloadDolphin()
            await unmount(modInstallElement)
        }

        setTimeout(() => {
            mickeyLogo.classList.toggle("MickeyLogoFinishedDisappear")
            background.classList.toggle("DisableBackground")

            setTimeout(() => {
                SetHeaderVisible(true)
                window.open("#/Games", "_self")
            }, 2000)
        }, 500)


    }

</script>
<div bind:this={background}
     class="DisableBackground"
     style="background-color:black;width:100vw;height:100vh;position:absolute;top:0;left:0;display: flex;justify-content: center;transition-duration: 1s;">
</div>
<div
        style="width:100vw;height:100vh;position:absolute;top:0;left:0;display: flex;justify-content: center;transition-duration: 1s;">
    <img alt="" bind:this={mickeyLogo} class="MickeyLogo MickeyLogoFinished MickeyLogoFinishedDisappear"
         src="img/emsvg.svg">
    <div bind:this={initialSetup} class="InitialSetup InitialSetupFinished"
         style="height: 100%;align-content: center;display: none;">
        <h2 style="text-align: center;">Welcome to Epic Mickey Launcher!</h2>
        <p></p>
        <span>• Install Dolphin (Required for Wii Emulation & Automatic Image Extraction): </span>
        <input
                bind:checked={installDolphin} type="checkbox"/>
        <p></p>
        <span style="width: 100%;justify-content: center;display: flex;">
            <button onclick={Proceed} style="text-align: center;">Get Started!</button>
            </span>
    </div>
</div>

<style>

    .MickeyLogo {
        align-self: center;
        position: absolute;
        height: 30%;
        transition-duration: 1s;
        opacity: 0;
    }

    .MickeyLogoFinished {
        height: 20%;
        opacity: 1;
    }

    .MickeyLogoFinishedDisappear {
        margin-bottom: 20%;
        opacity: 0;
    }

    .InitialSetup {
        opacity: 0;
        transition-duration: 1s;
    }

    .InitialSetupFinished {
        opacity: 1;
    }

    .DisableBackground {
        opacity: 0;
    }

</style>