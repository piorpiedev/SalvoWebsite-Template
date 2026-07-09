<script lang="ts">
  import { Router } from "sv-router";
  import { session } from "./session.svelte";
  import "sv-router/generated";

  session.fetch();
</script>

<Router />
