import { createHash } from 'node:crypto';

export default async (req) => {
  let password;
  try {
    const body = await req.json();
    password = body.password;
  } catch {
    return Response.json({ authorized: false });
  }

  if (!password || typeof password !== 'string') {
    return Response.json({ authorized: false });
  }

  const expectedHash = Netlify.env.get('INSTALLER_PASSWORD_HASH');
  if (!expectedHash) {
    return Response.json({ authorized: false });
  }

  const hash = createHash('sha256').update(password).digest('hex');

  return Response.json({ authorized: hash === expectedHash });
};

export const config = {
  path: '/api/verify-password',
  method: 'POST',
};
