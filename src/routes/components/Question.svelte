<script lang="ts">
  interface Props {
    content?: string;
  }

  let { content = "" }: Props = $props();
</script>

<img
  alt="Information"
  src="img/question.svg"
  style="width:15px;cursor:help;"
  title={content}
/>
