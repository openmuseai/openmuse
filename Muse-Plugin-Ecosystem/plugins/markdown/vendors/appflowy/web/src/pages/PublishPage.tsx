import { useParams } from 'react-router-dom';

import NotFound from '@/components/error/NotFound';
import { PublishView } from '@/components/publish';

function PublishPage() {
  const { namespace, publishName } = useParams();

  if (!namespace || !publishName) return <NotFound />;
  return <PublishView namespace={namespace} publishName={publishName} />;
}

export default PublishPage;
