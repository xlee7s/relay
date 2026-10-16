import { BookOutlined } from '@ant-design/icons';
import { Card, Empty } from 'antd';
import readme from '../../../README.md?raw';
import { useI18n } from '../i18n/context';
import { extractInstallDocumentation, renderReadmeDocumentation } from '../utils/readmeDocumentation';

const documentation = extractInstallDocumentation(readme);

export default function Documentation() {
  const { t } = useI18n();

  return (
    <>
      <h2 className="rp-page-title"><BookOutlined /> {t('documentation')}</h2>
      <Card className="rp-doc-card">
        {documentation ? (
          <article className="rp-doc-content">{renderReadmeDocumentation(documentation)}</article>
        ) : (
          <Empty description={t('documentationUnavailable')} />
        )}
      </Card>
    </>
  );
}
