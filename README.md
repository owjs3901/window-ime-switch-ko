# window-ime-switch-ko

window-ime-switch-ko 는 windows 환경에서 한국어 혹은 영어 입력기인지 확인하고 이를 설정하기 위한 프로젝트입니다.

최종적으로는 VSCode에서 한글을 입력하다가 visual/normal mode 로 전환되었을 때 한글이 입력되며 전환에 어려움을 겪는 것을 해소하는 것을 목표로 개발되었습니다.

## VSVode Setting

```json
{
    "vim.autoSwitchInputMethod.enable": true,
    "vim.autoSwitchInputMethod.defaultIM": "0",
    "vim.autoSwitchInputMethod.obtainIMCmd": "path\\window-ime-switch.exe",
    "vim.autoSwitchInputMethod.switchIMCmd": "path\\window-ime-switch.exe {im}"
}
```

settings.json 에 위와 같이 설정한다면 정상적으로 작동합니다.

## Command

### 현재 IME 상태 확인

```bash
./window-ime-switch.exe
```

0이면 영어 1이면 한국어

### IME 상태 설정

```bash
./window-ime-switch.exe {0 or 1}
```


## 시연 영상 및 코드 해설

[![Video Label](http://img.youtube.com/vi/rw2ArEHxmk8/0.jpg)](https://youtu.be/rw2ArEHxmk8)
