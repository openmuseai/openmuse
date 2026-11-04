import {useEffect} from 'react'
import {setAgentEditorContext} from './runtime'

/** Retain the original editor's selection/flush contract without its AI chat. */
export function AiPanel(props: any) {
  useEffect(() => {
    setAgentEditorContext({slideIndex:props.current,selectedIds:props.selectedIds,beforeRun:props.onBeforeRun,goTo:async(index:number)=>{
      props.onNavigate(index)
      await new Promise<void>(resolve=>requestAnimationFrame(()=>requestAnimationFrame(()=>resolve())))
    }})
  },[props.current,JSON.stringify(props.selectedIds),props.onBeforeRun,props.onNavigate])
  return null
}
