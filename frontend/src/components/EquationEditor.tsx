import type { Ref, RefObject } from 'react';

import { ChangeEvent, forwardRef } from 'react';

type BaseEquationEditorProps = {
  equation: string;
  inline: boolean;
  setEquation: (equation: string) => void;
};

function EquationEditor(
  {
    equation,
    setEquation,
    inline,
  }: BaseEquationEditorProps,
  forwardedRef: Ref<HTMLInputElement | HTMLTextAreaElement>
): JSX.Element {
  const onChange = (event: ChangeEvent) => {
    setEquation((event.target as HTMLInputElement).value);
  };

  return inline &&
    forwardedRef instanceof HTMLInputElement ? (
    <span className='bg-muted'>
      <span className='text-left'>$</span>
      <input
        className='m-0 resize-none border-0 p-0 outline-0'
        value={equation}
        onChange={onChange}
        autoFocus={true}
        ref={forwardedRef as RefObject<HTMLInputElement>}
      />
      <span className='text-left'>$</span>
    </span>
  ) : (
    <div className='bg-muted'>
      <span className='text-left'>{'$$\n'}</span>
      <textarea
        className='m-0 w-full resize-none border-0 p-0 outline-0'
        value={equation}
        onChange={onChange}
        ref={forwardedRef as RefObject<HTMLTextAreaElement>}
      />
      <span className='text-left'>{'\n$$'}</span>
    </div>
  );
}

export default forwardRef(EquationEditor);
