<script lang="ts">
    import {parse} from "marked";
    import DOMPurify from "dompurify";
    import {Game, Platform, PublisherMod, PublisherModData, PublisherRemoteType,} from "./library/types";
    import Question from "./components/Question.svelte";
    import PublisherModNode from "./components/PublisherModNode.svelte";
    import {onMount} from "svelte";
    import {LoadConfig, LoadModPublisherConfig, SaveModPublisherConfig,} from "./library/config";
    import {open} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
    import {copyFile} from "@tauri-apps/plugin-fs";
    import {path} from "@tauri-apps/api";
    import {ReadFile, WriteFile} from "./library/configfiles";
    import {Octokit} from "octokit";

    let gitHubCheck: HTMLInputElement;

    let modNameInput: HTMLInputElement;
    let shortDescriptionInput: HTMLInputElement;
    let descriptionInput: HTMLTextAreaElement;
    let gameInput: HTMLSelectElement;
    let platformInput: HTMLSelectElement;
    let iconPath: string = "";
    let createModDialog: HTMLDialogElement;

    let modNameLength = 0;
    let shortDescriptionLength = 0;
    let descriptionLength = 0;

    let modPublisherData: PublisherModData[];

    onMount(async () => {
        let config = await LoadConfig();

        if (config.gitHubSecret.trim() == "") {
            gitHubCheck.disabled = true;
        }

        modPublisherData = await LoadModPublisherConfig();
    });

    async function SelectIcon() {
        const selectedPath = await open({
            title: "Select folder",
            filters: [
                {
                    name: "Images",
                    extensions: ["png", "jpeg", "jpg", "gif", "webp", "bmp"],
                },
            ],
            directory: false,
            multiple: false,
        });

        iconPath = selectedPath;
    }

    async function ConfirmInput() {
        let failed = false;
        let modName = modNameInput.value.trim();
        if (modName === "") {
            await alert("Mod Name is empty!");
            failed = true;
        }

        if (modName.length > 40) {
            await alert("Mod Name is too long!");
            failed = true;
        }

        let shortDescription = shortDescriptionInput.value.trim();

        if (shortDescription === "") {
            await alert("Short Description is empty!");
            failed = true;
        }

        if (shortDescription.length > 64) {
            await alert("Short Description is too long!");
            failed = true;
        }

        let description = descriptionInput.value.trim();

        if (description === "") {
            await alert("Description is empty!");
            failed = true;
        }

        if (description.length > 8096) {
            await alert("Description is too long!");
            failed = true;
        }

        let game = gameInput.value;
        let platform = platformInput.value;

        if (game == Game.EM1 && platform == Platform.PC) {
            await alert("Impossible combination! (EM1/PC)");
            failed = true;
        }

        if (game == Game.EMR && platform == Platform.Wii) {
            await alert("Impossible combination! (EMR/WII)");
            failed = true;
        }

        if (iconPath.trim() === "") {
            await alert("Icon has not been chosen!");
            failed = true;
        }
        let config = await LoadConfig();

        if (failed) return;

        const selectedPath = await open({
            title: "Select folder",
            directory: true,
            multiple: false,
        });

        await invoke("generate_mod_project", {
            game: game,
            platform: platform,
            path: selectedPath,
        });

        let modMetaString = await ReadFile(selectedPath + "/mod.json");
        let modMeta: PublisherMod = JSON.parse(modMetaString);

        modMeta.name = modName;
        modMeta.short_description = shortDescription;
        let extension = await path.extname(iconPath);
        modMeta.icon_path = "icon." + extension;

        let iconDestination = selectedPath + "/icon." + extension;

        await copyFile(iconPath, iconDestination);
        await WriteFile(selectedPath + "/mod.json", JSON.stringify(modMeta));
        await WriteFile(selectedPath + "/description.md", description);

        await invoke("init_repository", {path: selectedPath});

        if (gitHubCheck.value) {
            const octokit = new Octokit({
                auth: config.gitHubSecret,
            });

            await octokit.request("POST /user/repos", {
                name: modName,
                description: shortDescription,
                headers: {
                    "X-GitHub-Api-Version": "2022-11-28",
                },
            });
        }

        modPublisherData.push({
            projectPath: selectedPath,
            sshKey: "",
            remoteType: gitHubCheck.value
                ? PublisherRemoteType.GitHub
                : PublisherRemoteType.Git,
        });
        SaveModPublisherConfig(modPublisherData);
    }
</script>

<h1>Mod Publisher</h1>

<div style="display:flex;height:70vw;">
    <div
            style="background-color:rgb(20 20 20); height:100%; width:30%;position:relative;"
    >
        <PublisherModNode></PublisherModNode>
        <div style="width:100%;position: absolute;bottom:0px;">
            <button on:click={() => createModDialog.showModal()} style="width:100%;"
            >+
            </button
            >
        </div>
    </div>

    <span style="margin-left:1%;"></span>
    <div
            style="height:100%; width:69%;background-color:rgb(20 20 20);position: relative;"
    >
        <div style="background-color: rgb(30 30 30);width:100%;">
            <img
                    alt=""
                    src="img/waren.png"
                    style="width:150px;height:150px;vertical-align:middle;"
            />
            <div style="display: inline;">
                <span style="font-size:50px;margin-left:15px;">Awesome Edition</span>
            </div>
        </div>
        <div style="position:relative;background-color:rgb(35 35 35);padding:5px;">
            <button
                    style="background-color: transparent;border:none;"
                    title="Toggle visibility in Mod Market. | Invisible"
            >
                <svg
                        style="width:35px;vertical-align: middle;fill:white;margin-right:10px;"
                        viewBox="0 0 488.85 488.85"
                >
                    <path
                            d="M244.425,98.725c-93.4,0-178.1,51.1-240.6,134.1c-5.1,6.8-5.1,16.3,0,23.1c62.5,83.1,147.2,134.2,240.6,134.2
		s178.1-51.1,240.6-134.1c5.1-6.8,5.1-16.3,0-23.1C422.525,149.825,337.825,98.725,244.425,98.725z M251.125,347.025
		c-62,3.9-113.2-47.2-109.3-109.3c3.2-51.2,44.7-92.7,95.9-95.9c62-3.9,113.2,47.2,109.3,109.3
		C343.725,302.225,302.225,343.725,251.125,347.025z M248.025,299.625c-33.4,2.1-61-25.4-58.8-58.8c1.7-27.6,24.1-49.9,51.7-51.7
		c33.4-2.1,61,25.4,58.8,58.8C297.925,275.625,275.525,297.925,248.025,299.625z"
                    />
                </svg>
            </button>
            <button>View in Mod Page</button>
            <button>Trigger Update</button>
            <button>Publish Mod</button>
            <button>Download Source</button>
            <button>Open In Explorer</button>
            <button>Remote Settings</button>
            <span
                    style="font-size:20px;float:right;cursor:help;"
                    title="You have local file changes that have not been uploaded to the server."
            >*</span
            >
        </div>

        <p></p>
        <div style="width:100%;height:72%;overflow-y: scroll;">
      <span style="position: relative;left:35px;">
        {@html DOMPurify.sanitize(
            parse(
                "# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n# awesome\n",
            ),
        )}
      </span>
        </div>
        <span style="position: absolute;bottom:5px;left:5px;"
        >⏳ Status Message...</span
        >
    </div>

    <dialog bind:this={createModDialog} style="min-width: 300px;">
        <button on:click={() => createModDialog.close()}>Close</button>
        <h2>Create a new Mod</h2>

        <span style="font-size:10px;">Mod Name [{modNameLength}/40]<br/></span>
        <input
                bind:this={modNameInput}
                on:input={() => (modNameLength = modNameInput.value.trim().length)}
                placeholder="Enter name of Mod..."
                style="width:90%;"
                type="text"
        />
        <br/>
        <span style="font-size:10px;"
        >Short Description [{shortDescriptionLength}/64]<br/></span
        >
        <input
                bind:this={shortDescriptionInput}
                on:input={() =>
        (shortDescriptionLength = shortDescriptionInput.value.trim().length)}
                placeholder="Concise description of your mod..."
                style="width:90%;"
                type="text"
        />

        <br/>
        <span style="font-size:10px;"
        >Description (Markdown) [{descriptionLength}/8096]<br/></span
        >
        <textarea
                bind:this={descriptionInput}
                on:input={() =>
        (descriptionLength = descriptionInput.value.trim().length)}
                placeholder="Detailed description of your mod..."
                style="width:90%;"
        />
        <p></p>

        <span style="font-size:10px;">Game<br/></span>
        <div class="dropdown">
            <select bind:this={gameInput}>
                <option value={Game.EM1}>Epic Mickey 1</option>
                <option value={Game.EM2}>Epic Mickey 2</option>
                <option value={Game.EMR}>Epic Mickey Rebrushed</option>
            </select>
        </div>
        <p></p>
        <span style="font-size:10px;">Platform<br/></span>
        <div class="dropdown">
            <select bind:this={platformInput}>
                <option value={Platform.Wii}>Wii</option>
                <option value={Platform.PC}>PC</option>
            </select>
        </div>
        <p></p>

        <button on:click={SelectIcon}>Select Icon</button>
        <span style="font-size:10px;"><br/>{iconPath}</span>

        <p>
      <span style="font-size:10px;"
      >Create and Link GitHub repository to mod:
      </span>

            <input bind:this={gitHubCheck} type="checkbox"/>
            <Question
                    content="You must link a GitHub account to do this. You can do so by going to the Settings tab."
            ></Question>
            <br/>
            <button on:click={ConfirmInput}>Create Mod</button>
        </p>
    </dialog>
</div>
