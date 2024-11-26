/// This solution seems a bit like magic but derives directly from algebraic manipulation
///
/// Let the unknown start location be given by the vector (X, Y, Z)
/// Let the unknown start velocity be given by the vector (U, V, W)
/// Let each hail path have location (x_i, y_i, z_i) & velocity (u_i, v_i, w_i)
///
/// Therefore for each path these are true:
/// X + U.t_i = x_i + u_i.t_i
/// Y + V.t_i = y_i + v_i.t_i
/// Z + W.t_i = z_i + w_i.t_i
/// where t_i is the time for that path to intersect the unknown common path
///
///
/// Rearranging to make t_i the subject and then eliminating gives:
/// X - x_i = (u_i - U).t_i
/// Y - y_i = (v_i - V).t_i
/// Z - z_i = (w_i - W).t_i
///
/// (X - x_i) / (u_i - U) = (Y - y_i) / (v_i - V)
/// (Y - y_i) / (v_i - V) = (Z - z_i) / (w_i - W)
/// (Z - z_i) / (w_i - W) = (X - x_i) / (u_i - U)
///
/// (X - x_i).(v_i - V) = (Y - y_i).(u_i - U)
/// (Y - y_i).(w_i - W) = (Z - z_i).(v_i - V)
/// (Z - Z_i).(u_i - U) = (X - x_i).(w_i - W)
///
///
/// Multiplying out and moving the quadratic unknown terms to one side:
/// X.v_i - x_i.v_i - XV + x_i.V = Y.u_i - y_i.u_i - UY + y_i.U
/// Y.w_i - y_i.w_i - YW + y_i.W = Z.v_i - z_i.v_i - ZV + z_i.V
/// Z.u_i - z_i.u_i - ZU + z_i.U = X.w_i - x_i.w_i - XW + x_i.W
///
/// X.v_i - x_i.v_i + x_i.V - Y.u_i + y_i.u_i - y_i.U = XV - UY
/// Y.w_i - y_i.w_i + y_i.W - Z.v_i + z_i.v_i - z_i.V = YW - ZV
/// Z.u_i - z_i.u_i + z_i.U - X.w_i + x_i.w_i - x_i.W = ZU - XW
///
///
/// Take _any_ pair of hail paths allows us to cancel out the quadratic unknows:
/// X.v_i - x_i.v_i + x_i.V - Y.u_i + y_i.u_i - y_i.U = X.v_j - x_j.v_j + x_j.V - Y.u_j + y_j.u_j - y_j.U
/// Y.w_i - y_i.w_i + y_i.W - Z.v_i + z_i.v_i - z_i.V = Y.w_j - y_j.w_j + y_j.W - Z.v_j + z_j.v_j - z_j.V
/// Z.u_i - z_i.u_i + z_i.U - X.w_i + x_i.w_i - x_i.W = Z.u_j - z_j.u_j + z_j.U - X.w_j + x_j.w_j - x_j.W
///
///
/// Which can be ultimately rearranged into linear equations in the unknowns:
/// X(v_i - v_j) - Y(u_i - u_j) + V(x_i - x_j) - U(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Y(w_i - w_j) - Z(v_i - v_j) + W(y_i - y_j) - V(z_i - z_j) = y_i.w_i - z_i.v_i - y_j.w_j + z_j.v_j
/// Z(u_i - u_j) - X(w_i - w_j) + U(z_i - z_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
///
/// From here we have more than enough input hail paths (300) to generate equations on which we can do
/// integer Gaussian elimination to solve.
///
///
/// However, I noticed in my input data that I was given 3 hail paths with the same starting x_i and
/// u_i. I was also given many hail paths with repeated u_i, v_i or w_i. Using these facts, we can pick
/// pairs of paths with common values to cancel out some of the unknows in the linear equations to
/// make the solution much simpler to compute:
///
/// 1) Start with a pair of paths with common x_i and u_i, which allows us to cancel out 2 of the
///    unknowns, and if you follow the algebra in `get_x_unknowns` it can be shown that the unknown X
///    and U values must be the same as the common x_i and u_i of these coincident paths.
///
/// 2) Next, take a pair of paths that only have a common u_i, (and by definition a distinct pair of
///    x_i values). We can take these to cancel out one of the unknown terms, plus pass in the X and U
///    unknowns that we now know. This simplifies two of our linear equations directly into a pair of
///    equations, each with one of the outstanding velocity unknowns. These can therefore be directly
///    determined.
///
/// 3) Finally with X, U, V and W all known, we can take a pair of the original linear equations in Y
///    and Z (with X, not each other), substitue in all the known values and derive the last two
///    position unknowns.
///
///
/// Since we look for common x_i and u_i paths, and then just common u_i paths, it makes it
/// computationally easy to order the paths first by u_i and then by x_i. All pairs of paths that we
/// will be looking for will be adjacent in such an arrangement, which allows us to loop through
/// pairs of paths to find the coincident paths we are looking for at each stage
///
///
/// It did occur to me that other inputs may not have paths with coincident x_i and u_i. If the
/// repetition were in some other dimension, it would be easy enough to rotate the dimensions so
/// that the repeated dimension was in the x position.
///
/// If there are inputs with no repeated paths in both the location and velocity then we would need to
/// revert to the Gaussian Elimination identified above. This solution is not coded for that though
pub mod path;

pub use path::Path;

/// "Find pair of hailstones with same x co-ord and u velocity"
/// (x_i - x_j) & (u_i - u_j) are both zero
///
/// Start with the linear equations:
/// X(v_i - v_j) - Y(u_i - u_j) + V(x_i - x_j) - U(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Z(u_i - u_j) - X(w_i - w_j) + U(z_i - z_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
///
/// Apply the fact that x_i & x_j,  u_i & u_j are the same value:
/// Let x = x_i, x_j
/// Let u = u_i, u_j
///
/// X(v_i - v_j) - Y(u - u) + V(x - x) - U(y_i - y_j) = x.v_i - y_i.u - x.v_j + y_j.u
/// Z(u - u) - X(w_i - w_j) + U(z_i - z_j) - W(x - x) = z_i.u - x.w_i - z_j.u + x.w_j
///
/// X(v_i - v_j) - 0.Y + 0.V - U(y_i - y_j) = x(v_i - v_j) - u(y_i - y_j)
/// 0.Z - X(w_i - w_j) + U(z_i - z_j) - 0.W = u(z_i - z_j) - x(w_i - w_j)
///
/// X(v_i - v_j) - U(y_i - y_j) = x(v_i - v_j) - u(y_i - y_j)
/// X(w_i - w_j) - U(z_i - z_j) = x(w_i - w_j) - u(z_i - z_j)
///
///
/// Re-arrange to cancel out U terms:
/// U - u = X(v_i - v_j)/(y_i - y_j) - x(v_i - v_j)/(y_i - y_j)
/// U - u = X(w_i - w_j)/(z_i - z_j) - x(w_i - w_j)/(z_i - z_j)
///
/// X(v_i - v_j)/(y_i - y_j) - X(w_i - w_j)/(z_i - z_j) = x(v_i - v_j)/(y_i - y_j) - x(w_i - w_j)/(z_i - z_j)
/// X(v_i - v_j)(z_i - z_j) - X(w_i - w_j)(y_i - y_j) = x(v_i - v_j)(z_i - z_j) - x(w_i - w_j)(y_i - y_j)
/// X((v_i - v_j)(z_i - z_j) - (w_i - w_j)(y_i - y_j)) = x((v_i - v_j)(z_i - z_j) - (w_i - w_j)(y_i - y_j))
///
///
/// Simplify by stating the differences in each dimension as a single delta term:
/// Let dv = (v_i - v_j)
/// Let dz = (z_i - z_j)
/// Let dw = (w_i - w_j)
/// Let dy = (y_i - y_j)
/// X(dv.dz - dw.dy) = x(dv.dz - dw.dy)
///
/// Therefore, as long as dv.dz != dw.dy
/// This would be dividing by zero, which would invalidate the algebra following
/// X = x
///
///
/// Substituting X back above to solve for U:
/// U - u = x(v_i - v_j)/(y_i - y_j) - x(v_i - v_j)/(y_i - y_j)
/// U - u = x.dv/dy - x.dv/dy
/// U - u = 0
/// U = u
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn get_x_unknowns(paths: &[Path]) -> (i64, i64) {
    for win in paths.windows(2) {
        let p1 = &win[0];
        let p2 = &win[1];
        if p1.x == p2.x && p1.u == p2.u {
            let dv = p1.v - p2.v;
            let dz = p1.z - p2.z;
            let dw = p1.w - p2.w;
            let dy = p1.y - p2.y;
            assert_ne!(dv * dz, dw * dy);

            return (p1.x, p1.u);
        }
    }
    unreachable!()
}

/// "Find pair of hailstones with same u velocity but diff x co-ord"
/// (u_i - u_j) is zero
///
///
/// Start with the linear equations:
/// X(v_i - v_j) - Y(u_i - u_j) + V(x_i - x_j) - U(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Z(u_i - u_j) - X(w_i - w_j) + U(z_i - z_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
///
/// Substitute in known X and U:
/// x(v_i - v_j) - Y(u_i - u_j) + V(x_i - x_j) - u(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Z(u_i - u_j) - x(w_i - w_j) + u(z_i - z_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
/// -Y(u_i - u_j) + V(x_i - x_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j - x(v_i - v_j) + u(y_i - y_j)
/// Z(u_i - u_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j + x(w_i - w_j) - u(z_i - z_j)
///
///
/// Apply the fact that u_i and u_j are the same value:
/// Let u' = u_i, u_j
///
/// -Y(u' - u') + V(x_i - x_j) = x_i.v_i - y_i.u' - x_j.v_j + y_j.u' - x(v_i - v_j) + u(y_i - y_j)
/// Z(u' - u') - W(x_i - x_j) = z_i.u' - x_i.w_i - z_j.u' + x_j.w_j + x(w_i - w_j) - u(z_i - z_j)
///
///
/// Simplify & rearrange:
/// -0.Y + V(x_i - x_j) = x_i.v_i - x_j.v_j - u'(y_i - y_j) - x(v_i - v_j) + u(y_i - y_j)
/// 0.Z - W(x_i - x_j) = -x_i.w_i + x_j.w_j + u'(z_i - z_j) + x(w_i - w_j) - u(z_i - z_j)
///
/// V(x_i - x_j) = x_i.v_i - x_j.v_j - u'(y_i - y_j) - x(v_i - v_j) + u(y_i - y_j)
/// -W(x_i - x_j) = -x_i.w_i + x_j.w_j + u'(z_i - z_j)+ x(w_i - w_j) - u(z_i - z_j)
///
/// V = (x_i.v_i - x_j.v_j - u'(y_i - y_j) - x(v_i - v_j) + u(y_i - y_j)) / (x_i - x_j)
/// W = (x_i.w_i - x_j.w_j - u'(z_i - z_j) - x(w_i - w_j) + u(z_i - z_j)) / (x_i - x_j)
///
/// V = (x_i.v_i - x_j.v_j - x(v_i - v_j) + (u - u').(y_i - y_j)) / (x_i - x_j)
/// W = (x_i.w_i - x_j.w_j - x(w_i - w_j) + (u - u').(z_i - z_j)) / (x_i - x_j)
///
/// This directly solves the remaining two velocity unknowns
#[must_use]
pub fn get_velocity_unknowns(paths: &[Path], x: i64, u: i64) -> (i64, i64) {
    for win in paths.windows(2) {
        let p1 = &win[0];
        let p2 = &win[1];
        if p1.u == p2.u && p1.x != p2.x {
            let dv = p1.v - p2.v;
            let dw = p1.w - p2.w;
            let dx = p1.x - p2.x;
            let dy = p1.y - p2.y;
            let dz = p1.z - p2.z;

            let v = (p1.x * p1.v - p2.x * p2.v - x * dv + (u - p1.u) * dy) / dx;
            let w = (p1.x * p1.w - p2.x * p2.w - x * dw + (u - p1.u) * dz) / dx;
            return (v, w);
        }
    }
    unreachable!()
}

/// "Find pair of hailstones with diff u velocity and diff x co-ord"
///
/// Start with the linear equations:
/// X(v_i - v_j) - Y(u_i - u_j) + V(x_i - x_j) - U(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Z(u_i - u_j) - X(w_i - w_j) + U(z_i - z_j) - W(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
///
/// Substitute in known X, U, V & W:
/// x(v_i - v_j) - Y(u_i - u_j) + v(x_i - x_j) - u(y_i - y_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j
/// Z(u_i - u_j) - x(w_i - w_j) + u(z_i - z_j) - w(x_i - x_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j
///
///
/// Simplify & rearrange:
/// -Y(u_i - u_j) = x_i.v_i - y_i.u_i - x_j.v_j + y_j.u_j - x(v_i - v_j) + u(y_i - y_j) - v(x_i - x_j)
/// Z(u_i - u_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j + x(w_i - w_j) - u(z_i - z_j) + w(x_i - x_j)
///
/// Y(u_i - u_j) = y_i.u_i - x_i.v_i - y_j.u_j + x_j.v_j + x(v_i - v_j) - u(y_i - y_j) + v(x_i - x_j)
/// Z(u_i - u_j) = z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j + x(w_i - w_j) - u(z_i - z_j) + w(x_i - x_j)
///
/// Y = (y_i.u_i - x_i.v_i - y_j.u_j + x_j.v_j + x(v_i - v_j) - u(y_i - y_j) + v(x_i - x_j)) / (u_i - u_j)
/// Z = (z_i.u_i - x_i.w_i - z_j.u_j + x_j.w_j + x(w_i - w_j) - u(z_i - z_j) + w(x_i - x_j)) / (u_i - u_j)
#[must_use]
pub fn get_position_unknowns(paths: &[Path], x: i64, u: i64, v: i64, w: i64) -> (i64, i64) {
    for win in paths.windows(2) {
        let p1 = &win[0];
        let p2 = &win[1];
        if p1.u != p2.u && p1.x != p2.x {
            let du = p1.u - p2.u;
            let dv = p1.v - p2.v;
            let dw = p1.w - p2.w;
            let dx = p1.x - p2.x;
            let dy = p1.y - p2.y;
            let dz = p1.z - p2.z;

            let y = (p1.y * p1.u - p1.x * p1.v - p2.y * p2.u + p2.x * p2.v + x * dv - u * dy
                + v * dx)
                / du;
            let z = (p1.z * p1.u - p1.x * p1.w - p2.z * p2.u + p2.x * p2.w + x * dw - u * dz
                + w * dx)
                / du;
            return (y, z);
        }
    }
    unreachable!()
}
