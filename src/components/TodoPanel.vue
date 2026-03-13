<template>
  <div class="todo-panel">
    <n-empty v-if="todos.length === 0" description="暂无待办事项" />
    <div v-else class="todo-list">
      <div
        v-for="(todo, index) in todos"
        :key="index"
        class="todo-item"
        :class="{ done: todo.done }"
      >
        <n-checkbox
          :checked="todo.done"
          @update:checked="toggleTodo(index)"
        />
        <span class="text">{{ todo.text }}</span>
        <n-button size="tiny" text @click="removeTodo(index)">
          <template #icon><n-icon :component="CloseOutline" /></template>
        </n-button>
      </div>
    </div>
    
    <n-input-group style="margin-top: 12px">
      <n-input
        v-model:value="newTodo"
        placeholder="添加待办..."
        @keydown.enter="addTodo"
      />
      <n-button type="primary" @click="addTodo">添加</n-button>
    </n-input-group>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NEmpty, NCheckbox, NButton, NInput, NInputGroup, NIcon } from 'naive-ui'
import { CloseOutline } from '@vicons/ionicons5'

interface Todo {
  text: string
  done: boolean
}

const todos = ref<Todo[]>([
  { text: '配置 DeepSeek API Key', done: false },
  { text: '测试古文翻译', done: false },
  { text: '添加更多词库数据', done: false },
])

const newTodo = ref('')

function addTodo() {
  if (newTodo.value.trim()) {
    todos.value.push({ text: newTodo.value.trim(), done: false })
    newTodo.value = ''
  }
}

function toggleTodo(index: number) {
  todos.value[index].done = !todos.value[index].done
}

function removeTodo(index: number) {
  todos.value.splice(index, 1)
}
</script>

<style scoped>
.todo-panel {
  padding: 8px;
}

.todo-list {
  max-height: 300px;
  overflow-y: auto;
}

.todo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  margin-bottom: 4px;
}

.todo-item:hover {
  background: #f5f5f5;
}

.todo-item.done .text {
  text-decoration: line-through;
  color: #999;
}

.todo-item .text {
  flex: 1;
}
</style>