<script>
    function OpenPage(page) {
        window.open("#/" + page, "_self");
    }

    document.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });
</script>

<main>
    <div class="header">
        <img
            src="/img/emlLogo.png"
            alt=""
            style="display: block; margin-left: auto; margin-right: 10px;z-index:1;"
        />
        <button on:click={() => OpenPage("modmarket")} class="headerButton"
            >Mod Market</button
        >
        <button on:click={() => OpenPage("games")} class="headerButton"
            >Games</button
        >
        <button on:click={() => OpenPage("settings")} class="headerButton"
            >Settings</button
        >
    </div>

    <p style="margin-bottom:60px" />
</main>

<style>
    .header {
        display: flex;
        columns: 1rem 1rem;
        width: 100%;
    }
    .headerButton {
        z-index: 1;
        width: 50%;
        border: 1px solid;
        border-color: rgb(138, 138, 138);
        background-color: rgb(82, 82, 82);
        transition-duration: 0.1s;
    }
    .headerButton:hover {
        background-color: rgb(43, 43, 43);
    }
</style>
