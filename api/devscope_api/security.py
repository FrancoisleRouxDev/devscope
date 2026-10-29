"""Request authentication: a per-launch bearer token shared only with Electron.

PURPOSE
    The API listens on localhost, but "localhost" is not a security boundary:
    any other program on the machine — or any web page in the user's browser
    doing ``fetch("http://127.0.0.1:<port>/scans", ...)`` — could otherwise
    ask devscope to read arbitrary folders. Every route except ``/health``
    therefore requires ``Authorization: Bearer <token>``, where the token is
    generated fresh by Electron at each launch and never written to disk.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# import secrets
# from typing import Annotated
#
# from fastapi import Depends, HTTPException, Request, status
# from fastapi.security import HTTPAuthorizationCredentials, HTTPBearer
#
# _bearer = HTTPBearer(auto_error=False)
#
#
# def require_token(
#     request: Request,
#     creds: Annotated[HTTPAuthorizationCredentials | None, Depends(_bearer)],
# ) -> None:
#     expected = request.app.state.settings.token.get_secret_value()
#     # compare_digest → constant-time comparison, no timing side channel.
#     if creds is None or not secrets.compare_digest(creds.credentials, expected):
#         raise HTTPException(status.HTTP_401_UNAUTHORIZED, "invalid or missing token")
#
#
# # Usage in a router:  router = APIRouter(dependencies=[Depends(require_token)])
