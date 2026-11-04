import {useState,useSyncExternalStore} from 'react'
import {askAgent} from './runtime'
import {presentation} from './presentation'
export function AgentButton() {
  useSyncExternalStore(presentation.subscribe,()=>presentation.revision)
  const [pending,setPending]=useState(false)
  const [error,setError]=useState('')
  if(!presentation.features.agent)return null
  return <div className="openmuse-agent-entry">
    <button className="rb-big" disabled={pending} onClick={async()=>{
      setPending(true);setError('')
      try {await askAgent(presentation.integration('agentPrompt'))}
      catch(e){setError((e as {code?:string})?.code??'operationFailed')}finally{setPending(false)}
    }}><span className="rb-big-icon" aria-hidden="true">✦</span><span>{presentation.integration(pending?'agentConnecting':'agent')}</span></button>
    {error&&<span role="alert">{presentation.integration(error)}</span>}
  </div>
}
