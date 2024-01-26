<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { readTextFile, BaseDirectory, writeTextFile } from '@tauri-apps/api/fs'
import { invoke } from '@tauri-apps/api'
import { Input, message, Modal } from 'ant-design-vue'
const showModal = ref(false)
const info = ref({
  username: '',
  password: ''
})
const INFO_PATH = 'info.txt'
async function handleInputInfo() {
  if (info.value.username === '' || info.value.password === '') {
    message.warning({
      content: '请输入用户名和密码',
      duration: 2
    })
    return
  }
  await writeTextFile(
    INFO_PATH,
    `${info.value.username}\n${info.value.password}`,
    {
      dir: BaseDirectory.App
    }
  )
  showModal.value = false
  handleSendEvent()
}

function handleLoading() {
  const loadingEl = document.querySelector(
    '.app-loading>.app-loading-wrap>.app-loading-title>.loading'
  ) as HTMLDivElement
  let loadingContent = ''
  const loadingText: string = '企知道登录中...'
  for (let index = 0; index < loadingText.length; index++) {
    const element = loadingText[index]
    loadingContent += `<div class="l-${index} letter">${element}</div>`
  }
  loadingEl.innerHTML = loadingContent
}

async function handleSendEvent() {
  try {
    const content = await readTextFile(INFO_PATH, {
      dir: BaseDirectory.App,
      append: true
    })
    const info = content.split('\n')
    if (info.length < 2) {
      showModal.value = true
    } else {
      invoke('handle_window', {
        payload: `
        document.addEventListener('DOMContentLoaded', () => {
          const accessTokenCookie = document.cookie.split(';').filter(item => item.includes('accessToken'))
          console.log({accessTokenCookie})
          if (accessTokenCookie.length > 0) {
            const accessToken = accessTokenCookie[0].split('=')[1]
            if (accessToken) {
              window.location.href = 'https://www.qizhidao.com'
            } else {
              setTimeout(() => {
                run()
              }, 500)
            }
          } else {
            setTimeout(() => {
              run()
            }, 500)
          }
        })
        async function run() {
          function sleep(ms) {
            return new Promise((resolve) => setTimeout(resolve, ms))
          }
          const spanEl = document.querySelector(
            '#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.tab-box > div:nth-child(2) > span'
          )
          spanEl.click()
          await sleep(500)
          const inputEl = document.querySelector(
            '#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div:nth-child(1) > div > div > input'
          )
          inputEl.value = ''
          await sleep(500)
          inputEl.value = '${info[0]}'
          inputEl.dispatchEvent(new Event('input'))
          await sleep(500)
          const passwordEl = document.querySelector(
            '#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.el-form-item.psput.is-required > div > div > input'
          )
          passwordEl.value = '${info[1]}'
          passwordEl.dispatchEvent(new Event('input'))
          await sleep(500)
          const ruleEl = document.querySelector(
            '#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.note-line > span > span > div > label > span > span'
          )
          ruleEl.click()
          await sleep(500)
          const loginEl = document.querySelector(
            '#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.el-form-item.btn-line > div > button'
          )
          loginEl.click()
        }
        `
      })
    }
  } catch (error) {
    showModal.value = true
  }
}

onMounted(async () => {
  handleLoading()
})

document.addEventListener('DOMContentLoaded', async () => {
  handleSendEvent()
})
</script>
<template>
  <div class="app-loading">
    <div v-if="!showModal" class="app-loading-wrap">
      <div class="app-loading-title">
        <div class="loading">企知道登录中...</div>
      </div>
    </div>
    <Modal
      v-model:open="showModal"
      title="登录信息"
      @ok="handleInputInfo"
      @cancel="
        () => {
          showModal = true
        }
      "
      :keyboard="false"
      :maskClosable="false"
      :closable="false"
    >
      <Input v-model:value="info.username" placeholder="请输入用户名" />
      <Input v-model:value="info.password" placeholder="请输入密码" />
    </Modal>
  </div>
</template>
<style>
html,
body,
#app {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

.app-loading {
  display: flex;
  width: 100%;
  height: 100%;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  background-color: #fff;
}
.dark .app-loading {
  background-color: #1a202c;
}
.app-loading .app-loading-wrap {
  position: absolute;
  top: 50%;
  left: 50%;
  display: flex;
  transform: translate3d(-50%, -50%, 0);
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
.app-loading-title {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  font-size: 30px;
  font-weight: 700;
}
.app-loading-title .loading {
  margin-top: 10px;
  display: flex;
}
.app-loading-title img {
  width: 60px;
  height: 60px;
  margin-bottom: 5px;
  animation-name: loading;
  animation-duration: 2s;
  animation-iteration-count: infinite;
  animation-direction: linear;
  animation-delay: 1.8s;
}
.loading .letter {
  animation-name: loading;
  animation-duration: 1.6s;
  animation-iteration-count: infinite;
  animation-direction: linear;
  float: left;
  font-size: 30px;
  color: #777;
}
.loading .l-0 {
  animation-delay: 0.48s;
}
.loading .l-1 {
  animation-delay: 0.6s;
}
.loading .l-2 {
  animation-delay: 0.72s;
}
.loading .l-3 {
  animation-delay: 0.84s;
}
.loading .l-4 {
  animation-delay: 0.96s;
}
.loading .l-5 {
  animation-delay: 1.08s;
}
.loading .l-6 {
  animation-delay: 1.2s;
}
.loading .l-7 {
  animation-delay: 1.32s;
}
.loading .l-8 {
  animation-delay: 1.44s;
}
.loading .l-9 {
  animation-delay: 1.56s;
}
.loading .l-10 {
  animation-delay: 1.68s;
}
.loading .l-11 {
  animation-delay: 1.8s;
}
.loading .l-12 {
  animation-delay: 1.92s;
}
.loading .l-13 {
  animation-delay: 2.04s;
}
.loading .l-14 {
  animation-delay: 2.16s;
}
@keyframes loading {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
</style>
