<script>
    import { SetData } from "../library/datatransfer";


let name;
let pfp;
let comment;

let accountid;

export function InitCommentNode(_comment, _pfp, _name, _accountid)
{
name = _name;
pfp = _pfp;
comment = _comment;
accountid = _accountid;
}

function OpenProfile(){
    SetData("profile_id", accountid);
    window.open("#/profilepage", "_self");
}

</script>

<div style="width:50%;height:100px;background-color:gray;border-radius:10px;display:flex;padding:3px;filter: drop-shadow(1px 1px 3px black);margin:auto;">
    <span style="margin-top:auto;margin-bottom:auto;text-align:center;margin-left:10px;">
        <img src={pfp} alt="" style="width:60px;height:60px;border-radius:100%;filter: drop-shadow(1px 1px 3px black);">
        <br>
        <button on:click={OpenProfile} class="hyperlinkbutton">{name}</button>
    </span>

    <span style="margin-left:20px;text-align:left;">
        {comment}
    </span>
</div>
<p></p>