using MatrixEngine.Framework;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.Utilities;
using SFML.System;
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
                v.X -= App.DeltaTimeAsSeconds * v.X.Sign() * nonstatic.velocityDrag.X;
                if (v.X.Sign() != nonstatic.Velocity.X.Sign()) {
                    v.X = 0;
                }
                v.Y -= App.DeltaTimeAsSeconds * v.Y.Sign() * nonstatic.velocityDrag.Y;
                if (v.Y.Sign() != nonstatic.Velocity.Y.Sign()) {
                    v.Y = 0;
                }
                nonstatic.Velocity = v;
            }

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

            nonstatic.Position += new Vector2f(x, 0) * App.DeltaTimeAsSeconds;
            var nonstatic_rect = nonstatic.Transform.fullRect;

            if (nonstatic.ColliderComponent.colliderType != ColliderComponent.ColliderType.None) {
                foreach (var rect in l) {
                    if (nonstatic_rect.isColliding(rect)) {
                        //if (rect.Y == -1) {
                        //    System.Console.WriteLine("?????????????");
                        //}

                        if (nonstatic_rect.cX < rect.cX) {
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

            nonstatic.Position += new Vector2f(0, y) * App.DeltaTimeAsSeconds;
            var nonstatic_rect = nonstatic.Transform.fullRect;

            if (nonstatic.ColliderComponent.colliderType != ColliderComponent.ColliderType.None) {
                foreach (var rect in l) {
                    if (nonstatic_rect.isColliding(rect)) {
                        if (nonstatic_rect.cY < rect.cY) {
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

            for (float i = 0; i < vel.X.Abs(); i += ContinuousStep) {
                if (UpdateRigidBodyHorizontaly(nonstatic, ContinuousStep * vel.X.Sign())) {
                    break;
                }
            }
            var v = vel.X % ContinuousStep;
            if (v != 0) {
                UpdateRigidBodyHorizontaly(nonstatic, v);
            }

            UpdateRigidBodyHorizontaly(nonstatic, vel.X % ContinuousStep);

            for (float i = 0; i < vel.Y.Abs(); i += ContinuousStep) {
                if (UpdateRigidBodyVerticly(nonstatic, ContinuousStep * vel.Y.Sign())) {
                    break;
                }
            }
            v = vel.Y % ContinuousStep;

            if (v != 0) {
                UpdateRigidBodyVerticly(nonstatic, v);
            }
        }

        private void AddTilemapToCollision(RigidBodyComponent nonstatic, ColliderComponent @static) {
            var tilemap = @static.GetComponent<TilemapComponent>();
            if (tilemap == null) {
                return;
            }

            var nonstatic_rect = nonstatic.ColliderComponent.Rect;
            var tile_scale = tilemap.Transform.scale;

            var list_rects = new List<Rect>();

            Vector2f pos;

            for (float x = -tile_scale.X * 2; x < nonstatic_rect.width + tile_scale.X * 2; x += tile_scale.X) {
                for (float y = -tile_scale.Y * 2; y < nonstatic_rect.height + tile_scale.Y * 2; y += tile_scale.Y) {
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
    }
}