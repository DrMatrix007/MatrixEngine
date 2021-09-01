using MatrixEngine.Framework;
using MatrixEngine.Framework.MathM;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        private const float ContinuousStep = 0.01f;

        public const float Threshold = 0.010f;

        private List<bool> isUp;
        private List<bool> isLeft;


        private List<RigidBodyComponent> dynamicRigidBodiesToCalc;
        private List<ColliderComponent> collidersToCalc;

        private List<Rect> rectsToCalc;


        public Framework.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(Framework.App app) {
            this.app = app;
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





            var static_list = collidersToCalc.ToArray();
            var non_static_list = dynamicRigidBodiesToCalc.ToArray();



            foreach (var @static in static_list) {
                @static.isGrounded = false;
                if (@static.colliderType == ColliderComponent.ColliderType.None) {
                    continue;
                }

                foreach (var nonstatic in non_static_list) {

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.None) {
                        continue;
                    }

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {

                        if (@static.colliderType == ColliderComponent.ColliderType.Rect) {

                            AddRectToCollision(@static.rect);

                        }
                        if (@static.colliderType == ColliderComponent.ColliderType.Tilemap) {
                            AddTilemapToCollision(nonstatic, @static);
                        }


                    }

                }
            }

            var rs = rectsToCalc.ToArray();
            Vector2f vel;

            foreach (var nonstatic in dynamicRigidBodiesToCalc) {



                UpdateRigidBody(nonstatic, nonstatic.velocity);


                var add_to_vel = (nonstatic.gravity * app.deltaTime);

                //add_to_vel += (nonstatic.gravity * app.deltaTime);


                nonstatic.velocity += add_to_vel;

                var v = nonstatic.velocity;
                v.X -= app.deltaTime * v.X.Sign() * nonstatic.velocityDrag.X;
                if (v.X.Sign() != nonstatic.velocity.X.Sign()) {
                    v.X = 0;
                }
                v.Y -= app.deltaTime * v.Y.Sign() * nonstatic.velocityDrag.Y;
                if (v.Y.Sign() != nonstatic.velocity.Y.Sign()) {
                    v.Y = 0;
                }
                nonstatic.velocity = v;












            }

            foreach (var collider in rectsToCalc) {
                var rect = collider;
                var s = new RectangleShape();

                s.Position = rect.position;
                s.Size = rect.size;
                s.FillColor = Color.Red;

                //app.window.Draw(s);

                s.Dispose();
            }

            dynamicRigidBodiesToCalc.Clear();
            collidersToCalc.Clear();
            rectsToCalc.Clear();
        }

        bool UpdateRigidBodyHorizontaly(RigidBodyComponent nonstatic, float x) {

            if (x == 0) {
                return false;
            }
            var nonstatic_rect = nonstatic.transform.fullRect;


            var l = rectsToCalc
    //.Where(e => !e.isColliding(nonstatic_rect))
    .ToList();


            nonstatic.position += new Vector2f(x, 0) * app.deltaTime;
            nonstatic_rect = nonstatic.transform.fullRect;



            foreach (var rect in l) {



                if (nonstatic_rect.isColliding(rect)) {
                    //if (rect.Y == -1) {
                    //    System.Console.WriteLine("?????????????");
                    //}

                    if (nonstatic_rect.cX < rect.cX) {
                        nonstatic.position = new Vector2f(rect.X - nonstatic_rect.width, nonstatic.position.Y);
                    } else {
                        nonstatic.position = new Vector2f(rect.max.X, nonstatic.position.Y);
                    }
                    return true;
                }
            }
            return false;
        }

        private bool UpdateRigidBodyVerticly(RigidBodyComponent nonstatic, float y) {

            if (y == 0) {
                return false;
            }

            var nonstatic_rect = nonstatic.transform.fullRect;


            var l = rectsToCalc
                //.Where(e => !e.isColliding(nonstatic_rect))
                .ToList();

            nonstatic.position += new Vector2f(0, y) * app.deltaTime;
            nonstatic_rect = nonstatic.transform.fullRect;
            
            
            nonstatic.colliderComponent.isGrounded = false;



            foreach (var rect in l) {



                if (nonstatic_rect.isColliding(rect)) {




                    if (nonstatic_rect.cY < rect.cY) {
                        nonstatic.position = new Vector2f(nonstatic.position.X, rect.Y - nonstatic_rect.height);
                        nonstatic.colliderComponent.isGrounded = true;
                        nonstatic.velocity = nonstatic.velocity.OnlyWithX();
                    } else {
                        nonstatic.position = new Vector2f(nonstatic.position.X, rect.max.Y);
                        nonstatic.velocity = nonstatic.velocity.OnlyWithX();
                    }
                    return true;
                }
            }
            return false;
        }

        private void UpdateRigidBody(RigidBodyComponent nonstatic, Vector2f vel) {


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

            var nonstatic_rect = nonstatic.colliderComponent.rect;
            var tile_scale = tilemap.transform.scale;

            var list_rects = new List<Rect>();

            var pos = new Vector2f(0, 0);

            for (float x = -tile_scale.X * 2; x < nonstatic_rect.width + tile_scale.X * 2; x += tile_scale.X) {
                for (float y = -tile_scale.Y * 2; y < nonstatic_rect.height + tile_scale.Y * 2; y += tile_scale.Y) {
                    pos = new Vector2f(x, y) + nonstatic.position;
                    if (tilemap.GetTileFromWorldPos(pos ) != null) {
                        var r = new Rect(((Vector2f)tilemap.GetPosOfTileFromWorldPos(pos)).Multiply(tile_scale)+tilemap.position, tile_scale);
                        list_rects.Add(r);

                    }
                }
            }
            foreach (var item in list_rects) {
                AddRectToCollision(item);
            }
        }
        void AddRectToCollision(Rect @static) {
            rectsToCalc.Add(@static);
        }
    }

}

