<template>
  <div
    class="w-0 h-0"
    :style="{
      'border-top': `${height / 2}px solid transparent`,
      'border-bottom': `${height / 2}px solid transparent`,
      'border-left': direction === 'right' ? `${width}px solid ${color}` : undefined,
      'border-right': direction === 'left' ? `${width}px solid ${color}` : undefined,
    }"
  ></div>
</template>

<script lang="ts">
  export const triangleDirections = ['right', 'left'];
  export type TriangleDirection = typeof triangleDirections[number];
</script>

<script setup lang="ts">
  const { direction = triangleDirections[0] } = defineProps<{
    height: number;
    width: number;
    direction?: TriangleDirection;
    color: string;
  }>();
</script>
