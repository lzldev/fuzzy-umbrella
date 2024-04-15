import { createFileRoute } from '@tanstack/react-router'
import { ofetch } from 'ofetch'
import { useEffect, useRef } from 'react'
import { FlexContainer } from '~/components/FlexContainer'

export const Route = createFileRoute('/funny')({
  component: FileUploadForm,
})

function FileUploadForm() {
  const fileRef = useRef<HTMLInputElement>(null!)

  const submit = async () => {
    const d = await ofetch<
      {
        url:string,
        fields:Record<string,string>
      }
    >('http://localhost:3000/api/posts/create', {
      method: 'POST',
      credentials: 'include',
    })

    const formData = new FormData()

     for( const [key, value] of Object.entries(d.fields)){
      formData.append(key, value)
    }

    formData.append('file', fileRef.current.files?.item(0)!)

    const d2 = fetch(d.url, {
      method: 'POST',
      body: formData,
    })
    // const d2 = await ofetch(d.url,{
    //   method: 'POST',
    //   body: formData,
    //   headers:{
    //     'Content-Type': 'multipart/form-data'
    //   }
    // })

    console.log(d2)
  }

  const onpress = () => {
    console.log(fileRef.current.files?.item(0))
  }

  return (
    <FlexContainer>
      <div className="flex flex-col gap-2">
        <input type="text" name="text" />
        <input ref={fileRef} type="file" name="file" id="file" size={200} />
        <button onClick={submit}>Submit</button>


        <div onClick={submit}>Send File</div>
        <div onClick={onpress}>Print File</div>
        <div>
          <button
            type="button"
            onClick={async () => {
              await ofetch('http://localhost:3000/ping', {
                credentials: 'include',
              })
            }}
          >
            Send
          </button>
        </div>
      </div>
    </FlexContainer>
  )
}
