<script lang="ts">
  import { Button } from "$lib/components/ui/button";

  const paths = [
    { label: "Home", path: "/" },
    { label: "Library", path: "/client" },
    { label: "Config", path: "/settings" },
  ];
</script>

<div class="flex gap-1 rounded-md border p-1">
  {#each paths as path (path.path)}
    <a href={path.path}>
      <Button variant="ghost" size="sm">{path.label}</Button>
    </a>
  {/each}
</div>
