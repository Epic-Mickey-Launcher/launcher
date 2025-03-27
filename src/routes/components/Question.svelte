<script lang="ts">
  interface Props {
    content?: string;
  }

  let { content = "" }: Props = $props();
</script>

<img
  src="img/question.svg"
  alt=""
  title={content}
  style="width:15px;cursor:help;"
/>
