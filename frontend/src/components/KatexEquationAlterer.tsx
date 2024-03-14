import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useCallback, useState } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import KatexRenderer from './KatexRenderer';
import { Button } from './ui/button';

type Props = {
  initialEquation?: string;
  onConfirm: (equation: string, inline: boolean) => void;
};

export default function KatexEquationAlterer({
  onConfirm,
  initialEquation = '',
}: Props): JSX.Element {
  const [editor] = useLexicalComposerContext();
  const [equation, setEquation] =
    useState<string>(initialEquation);
  const [inline, setInline] = useState<boolean>(true);

  const onClick = useCallback(() => {
    onConfirm(equation, inline);
  }, [onConfirm, equation, inline]);

  const onCheckboxChange = useCallback(() => {
    setInline(!inline);
  }, [setInline, inline]);

  return (
    <>
      <div className='my-2 flex flex-row justify-between overflow-hidden'>
        Inline
        <input
          type='checkbox'
          checked={inline}
          onChange={onCheckboxChange}
        />
      </div>
      <div className='my-2 flex flex-row justify-between overflow-hidden'>
        Equation{' '}
      </div>
      <div className='my-2 flex flex-row justify-center overflow-hidden'>
        {inline ? (
          <input
            onChange={(event) => {
              setEquation(event.target.value);
            }}
            value={equation}
            className='w-full resize-none p-2'
          />
        ) : (
          <textarea
            onChange={(event) => {
              setEquation(event.target.value);
            }}
            value={equation}
            className='w-full resize-none p-2'
          />
        )}
      </div>
      <div className='my-2 flex flex-row justify-between overflow-hidden'>
        Visualization{' '}
      </div>
      <div className='my-2 flex flex-row justify-center overflow-hidden'>
        <ErrorBoundary
          onError={(e) => editor._onError(e)}
          fallback={null}
        >
          <KatexRenderer
            equation={equation}
            inline={false}
            onDoubleClick={() => null}
          />
        </ErrorBoundary>
      </div>
      {/* TODO: change classname */}
      <div className='KatexEquationAlterer_dialogActions'>
        <Button onClick={onClick}>Confirm</Button>
      </div>
    </>
  );
}
