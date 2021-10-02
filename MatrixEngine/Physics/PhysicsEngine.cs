using MatrixEngine.Framework;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.Utilities;
using SFML.Graphics;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Physics {

    public class PhysicsEngine {
        private const float ContinuousStep = 0.1f;

        public const float Threshold = 0.010f;

        private readonly List<RigidBodyComponent> dynamicRigidBodiesToCalc;
        private readonly List<ColliderComponent> collidersToCalc;

        private readonly List<Rect> rectsToCalc;
        private ColliderComponent[] static_list;
        private RigidBodyComponent[] non_static_list;

        public Framework.App App
        {
            get;
            private set;
        }

        public PhysicsEngine(Framework.App app) {
            this.App = app;
            dynamicRigidBodiesToCalc = new List<RigidBodyComponent>();
            collidersToCalc = new List<ColliderComponent>();
            rectsToCalc = new List<Rect>();
        }

        public void AddRigidbodyToFrame(RigidBodyComponent rigidBodyComponent) {
            dynamicRigidBodiesToCalc.Add(rigidBodyComponent);
        }

        public void AddColliderToFrame(ColliderComponent rect) {
            collidersToCalc.Add(rect);
        }

        public void Update() {
            // foreach (var nonstatic in dynamicRigidBodiesToCalc) {
            //     if (!nonstatic.isStatic) {
            //
            //
            //
            //     }
            // }

            static_list = collidersToCalc.ToArray();
            non_static_list = dynamicRigidBodiesToCalc.ToArray();

            foreach (var @static in static_list) {
                if (@static.colliderType == ColliderComponent.ColliderType.None) {
                    continue;
                }

                foreach (var nonstatic in non_static_list) {
                    if (nonstatic.ColliderComponent.colliderType == ColliderComponent.ColliderType.None) {
                        continue;
                    }
                    //nonstatic.ClearTouches();

                    if (nonstatic.ColliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {
                        if (@static.colliderType == ColliderComponent.ColliderType.Rect) {
                            AddRectToCollision(@static.Rect);
                        }
                        if (@static.colliderType == ColliderComponent.ColliderType.Tilemap) {
                            AddTilemapToCollision(nonstatic, @static);
                        }
                    }
                }
            }

            foreach (var nonstatic in dynamicRigidBodiesToCalc) {
                UpdateRigidBody(nonstatic, nonstatic.Velocity);

                var add_to_vel = (nonstatic.gravity * App.DeltaTimeAsSeconds);

                //add_to_vel += (nonstatic.gravity * app.deltaTime);

                nonstatic.Velocity += add_to_vel;
                var v = nonstatic.Velocity;
                v.X -= App.DeltaTimeAsSeconds * v.X.Sign() * nonstatic.VelocityDrag.X;
                if (v.X.Sign() != nonstatic.Velocity.X.Sign()) {
                    v.X = 0;
                }
                v.Y -= App.DeltaTimeAsSeconds * v.Y.Sign() * nonstatic.VelocityDrag.Y;
                if (v.Y.Sign() != nonstatic.Velocity.Y.Sign()) {
                    v.Y = 0;
                }
                nonstatic.Velocity = v;
            }

            //foreach (var nonstatic in dynamicRigidBodiesToCalc) {
            //    UpdateRigidBody(nonstatic, nonstatic.Velocity);

            //    var add_to_vel = (nonstatic.gravity * App.DeltaTimeAsSeconds);

            //    //add_to_vel += (nonstatic.gravity * app.deltaTime);

            //    nonstatic.Velocity += add_to_vel;

            //    var v = nonstatic.Velocity;
            //    v.X -= App.DeltaTimeAsSeconds * v.X.Sign() * nonstatic.VelocityDrag.X;
            //    if (v.X.Sign() != nonstatic.Velocity.X.Sign()) {
            //        v.X = 0;
            //    }
            //    v.Y -= App.DeltaTimeAsSeconds * v.Y.Sign() * nonstatic.VelocityDrag.Y;
            //    if (v.Y.Sign() != nonstatic.Velocity.Y.Sign()) {
            //        v.Y = 0;
            //    }
            //    nonstatic.Velocity = v;
            //}

            //foreach (var collider in rectsToCalc) {
            //    var rect = collider;
            //    var s = new RectangleShape();

            //    s.Position = rect.position;
            //    s.Size = rect.size;
            //    s.FillColor = Color.Red;

            //    app.window.Draw(s);

            //    s.Dispose();
            //}

            dynamicRigidBodiesToCalc.Clear();
            collidersToCalc.Clear();
            rectsToCalc.Clear();
        }

        private bool UpdateRigidBodyHorizontaly(RigidBodyComponent nonstatic, float x) {
            if (x == 0) {
                return false;
            }

            var l = rectsToCalc
    //.Where(e => !e.isColliding(nonstatic_rect))
    .ToList();
            var old_nonstatic_rect = nonstatic.Transform.fullRect;
            nonstatic.Position += new Vector2f(x, 0) * App.DeltaTimeAsSeconds;
            var nonstatic_rect = nonstatic.Transform.fullRect;

            if (nonstatic.ColliderComponent.colliderType != ColliderComponent.ColliderType.None) {
                foreach (var rect in l) {
                    if (nonstatic_rect.IsColliding(rect) || (nonstatic_rect.IsColliding(rect) && (old_nonstatic_rect.center.X < rect.center.X != nonstatic_rect.center.X < rect.center.X))) {
                        //if (rect.Y == -1) {
                        //    System.Console.WriteLine("?????????????");
                        //}

                        if (old_nonstatic_rect.cX < rect.cX) {
                            nonstatic.Position = new Vector2f(rect.X - nonstatic_rect.width, nonstatic.Position.Y);
                            nonstatic.TouchRight = true;
                        } else {
                            nonstatic.Position = new Vector2f(rect.max.X, nonstatic.Position.Y);
                            nonstatic.TouchLeft = true;
                        }
                        return true;
                    }
                }
            }
            return false;
        }

        private bool UpdateRigidBodyVerticly(RigidBodyComponent nonstatic, float y) {
            if (y == 0) {
                return false;
            }

            var l = rectsToCalc
                //.Where(e => !e.isColliding(nonstatic_rect))
                .ToList();
            var old_nonstatic_rect = nonstatic.Transform.fullRect;
            nonstatic.Position += new Vector2f(0, y) * App.DeltaTimeAsSeconds;
            var nonstatic_rect = nonstatic.Transform.fullRect;

            if (nonstatic.ColliderComponent.colliderType != ColliderComponent.ColliderType.None) {
                foreach (var rect in l) {
                    if (nonstatic_rect.IsColliding(rect) || (nonstatic_rect.IsColliding(rect) && (old_nonstatic_rect.center.X < rect.center.X != nonstatic_rect.center.X < rect.center.X))) {
                        if (old_nonstatic_rect.cY < rect.cY) {
                            nonstatic.Position = new Vector2f(nonstatic.Position.X, rect.Y - nonstatic_rect.height);
                            nonstatic.TouchDown = true;
                            nonstatic.Velocity = nonstatic.Velocity.OnlyWithX();
                        } else {
                            nonstatic.Position = new Vector2f(nonstatic.Position.X, rect.max.Y);
                            nonstatic.TouchUp = true;

                            nonstatic.Velocity = nonstatic.Velocity.OnlyWithX();
                        }
                        return true;
                    }
                }
            }

            return false;
        }

        private void UpdateRigidBody(RigidBodyComponent nonstatic, Vector2f vel) {
            nonstatic.ClearTouches();

            //for (float i = 0; i < vel.X.Abs(); i += ContinuousStep) {
            //    if (UpdateRigidBodyHorizontaly(nonstatic, ContinuousStep * vel.X.Sign())) {
            //        break;
            //    }
            //}
            //var v = vel.X % ContinuousStep;
            //if (v != 0) {
            //    UpdateRigidBodyHorizontaly(nonstatic, v);
            //}

            UpdateRigidBodyHorizontaly(nonstatic, vel.X);
            UpdateRigidBodyVerticly(nonstatic, vel.Y);

            //UpdateRigidBodyHorizontaly(nonstatic, vel.X % ContinuousStep);

            //for (float i = 0; i < vel.Y.Abs(); i += ContinuousStep) {
            //    if (UpdateRigidBodyVerticly(nonstatic, ContinuousStep * vel.Y.Sign())) {
            //        break;
            //    }
            //}
            //(vel.Y.Abs() / ContinuousStep).Log();
            //v = vel.Y % ContinuousStep;

            //if (v != 0) {
            //    UpdateRigidBodyVerticly(nonstatic, v);
            //}
        }

        private void AddTilemapToCollision(RigidBodyComponent nonstatic, ColliderComponent @static) {
            var tilemap = @static.GetComponent<TilemapComponent>();
            if (tilemap == null) {
                return;
            }

            var nonstatic_rect = nonstatic.ColliderComponent.Rect;
            var tile_scale = tilemap.Transform.Scale;

            var list_rects = new List<Rect>();

            Vector2f pos;

            var vx = nonstatic.Velocity.X * App.DeltaTimeAsSeconds;
            var vy = nonstatic.Velocity.Y * App.DeltaTimeAsSeconds;

            vx = vx.Abs() > 1 ? vx.Abs() : 1;
            vy = vy.Abs() > 1 ? vy.Abs() : 1;

            for (float x = -tile_scale.X * (vx+1); x < (nonstatic_rect.width + tile_scale.X) *( vx+1); x += tile_scale.X) {
                for (float y = -tile_scale.Y * (vy+1); y < (nonstatic_rect.height + tile_scale.Y) *( vy+1); y += tile_scale.Y) {
                    pos = new Vector2f(x, y) + nonstatic.Position;
                    if (tilemap.GetTileFromWorldPos(pos) != null) {
                        var r = new Rect(((Vector2f)tilemap.GetPosOfTileFromWorldPos(pos)).Multiply(tile_scale) + tilemap.Position, tile_scale);
                        list_rects.Add(r);
                    }
                }
            }
            foreach (var item in list_rects) {
                AddRectToCollision(item);
            }
        }

        private void AddRectToCollision(Rect @static) {
            rectsToCalc.Add(@static);
        }

        public Vector2f LineCast(Line line, Func<ColliderComponent, bool> check = null) {
            var points = new List<Vector2f>();
            foreach (var item in check != null ? collidersToCalc.Where(check).ToList() : collidersToCalc) {
                switch (item.colliderType) {
                    case ColliderComponent.ColliderType.None:
                        break;

                    case ColliderComponent.ColliderType.Rect:

                        points.AddRange(line.GetCollidingPosFromLineToRect(item.Transform.fullRect));

                        break;

                    case ColliderComponent.ColliderType.Tilemap:
                        var t = item.GetComponent<TilemapComponent>();
                        if (t != null) {
                            var step = t.Transform.Scale.X > t.Transform.Scale.Y ? t.Transform.Scale.X : t.Transform.Scale.Y;
                            //step /= 10f;
                            var isx = (line.start.X - line.end.X).Abs() > (line.start.Y - line.end.Y).Abs();
                            //Console.WriteLine(isx);

                            if (isx) {
                                for (float i = line.start.X; i.IsBetween(line.start.X, line.end.X); i += step * -(line.start.X - line.end.X).Sign()) {
                                    Vector2i pos;
                                    Tile tile;
                                    Vector2f rpos;
                                    for (int add = -5; add <= 3; add++) {
                                        pos = t.GetPosOfTileFromWorldPos(line.WhereX(i)) + new Vector2i(0, add);
                                        tile = t.GetTileFromTilemapPos(pos);
                                        if (tile == null) {
                                            continue;
                                        }

                                        rpos = t.GetWorldPosFromTilePos(pos);

                                        var s = new RectangleShape() { Position = rpos, Size = t.TileRect.size, FillColor = new Color(255, 255, 255, 100) };
                                        App.Window.Draw(s);
                                        s.Dispose();

                                        //var anspos = line.GetCollidingPosFromLineToRect(t.TileRect.SetPos(rpos));
                                        //points.AddRange(anspos);
                                        //if (anspos.IsFinite()) {
                                        //    points.Add(anspos);
                                        //    exit = true;
                                        //    //break;

                                        //}
                                        foreach (var lrect in t.TileRect.SetPos(rpos).ToLines()) {
                                            points.Add(line.GetCollidingPoint(lrect));
                                        }
                                    }
                                }
                            } else {
                                for (float i = line.start.Y; i.IsBetween(line.start.Y, line.end.Y); i += step * -(line.start.Y - line.end.Y).Sign()) {
                                    Vector2i pos;
                                    Tile tile;
                                    Vector2f rpos;
                                    bool exit = false;

                                    for (int add = -4; add <= 4; add++) {
                                        pos = t.GetPosOfTileFromWorldPos(line.WhereY(i)) + new Vector2i(add, 0);
                                        tile = t.GetTileFromTilemapPos(pos);
                                        if (tile == null) {
                                            continue;
                                        }
                                        rpos = t.GetWorldPosFromTilePos(pos);

                                        var s = new RectangleShape() { Position = rpos, Size = t.TileRect.size, FillColor = new Color(255, 255, 255, 100) };

                                        App.Window.Draw(s);
                                        s.Dispose();

                                        var r = t.TileRect.SetPos(rpos);

                                        foreach (var l in r.ToLines()) {
                                            var v = l.ToVertexArray();
                                            App.Window.Draw(v);
                                            v.Dispose();
                                        }

                                        var anspos = line.GetCollidingPosFromLineToRect(r);

                                        //points.AddRange(anspos);

                                        foreach (var lrect in t.TileRect.SetPos(rpos).ToLines()) {
                                            points.Add(line.GetCollidingPoint(lrect));
                                        }

                                        //if (anspos.IsFinite()) {
                                        //    points.Add(anspos);
                                        //    exit = true;
                                        //    //break;

                                        //}
                                    }
                                }
                            }

                            break;
                        }
                        break;

                    default:
                        break;
                }
            }
            if (points.Count == 0) {
                return line.end;
            }

            points.ForEach(e => {
                var r = new RectangleShape() { Position = e - new Vector2f(0.1f, 0.1f), Size = new Vector2f(0.2f, 0.2f), FillColor = Color.Red };

                App.Window.Draw(r);

                r.Dispose();
            });
            var ans = points.Aggregate((e, a) => line.start.Distance(a) < line.start.Distance(e) ? a : e);
            points.Clear();
            if (ans.X.IsInfinite() || ans.Y.IsInfinite()) {
                return line.end;
            }
            return ans;
        }
    }
}